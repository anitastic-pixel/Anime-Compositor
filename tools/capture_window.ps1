# Starts the shell, photographs its window, and stops it again.
#
# This is the one artifact in this repository that no test can produce. Everything under
# verification/ is written by `cargo test` and CI checks that the committed copy still matches;
# a window is not a value a test can compare, so it is captured here instead and the artifact
# note beside it says so. Run from the repository root:
#
#     cargo build -p anime_compositor_app --release
#     powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1
#
# Release, and not debug, because playback speed is one of the things these pictures show and an
# unoptimized build's frame rate says more about the compiler than about the renderer. -Build
# debug photographs the debug build instead.
#
# It writes verification/B-08_window_shell.png and prints the size it captured. -Name writes a
# different file; -Keys presses keys in the window first and -Settle waits that many milliseconds
# afterwards, which is how the playback screenshot is taken; -Ctrl holds Control down while those
# keys are pressed, which is how a Ctrl+S is photographed actually saving; -Open starts the shell
# on a project file, which is the same path a dropped file takes and the only one a script can
# drive.
#
# -Scale photographs the window as it would look on a display set to that scaling, by telling the
# web view what device scale factor to render at. It is not the same thing as changing the Windows
# display setting: the title bar and the window frame belong to Windows and stay where they are.
# What it does cover is everything inside the window, which is all of this application's interface,
# and it is the only half of the question a script can ask on its own.

param(
  [ValidateSet('release','debug')][string]$Build = 'release',
  [string]$Name = 'B-08_window_shell',
  [string]$Keys = '',
  [switch]$Ctrl,
  [int]$Settle = 1500,
  [string]$Open = '',
  [double]$Scale = 0
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "target\$Build\anime_compositor_app.exe"
$out = Join-Path $root "verification\$Name.png"

if (-not (Test-Path $exe)) { throw "build it first: cargo build -p anime_compositor_app --$Build" }

Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class Win {
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern short VkKeyScan(char c);
  [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
}
'@

# Without this, GetWindowRect answers in physical pixels while PowerShell thinks in scaled ones,
# and on a 150% display the shutter lands on a rectangle that is neither the window nor where it
# is. The first capture attempt photographed the desktop behind it.
[void][Win]::SetProcessDPIAware()

if ($Scale -gt 0) {
  # WebView2 reads its extra command line from this variable at startup. Set on this PowerShell
  # process so the child inherits it and nothing else on the machine is affected.
  $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--force-device-scale-factor=$Scale"
  # And a data folder of its own, because WebView2 shares one browser process per data folder and
  # a second window joining the first one's process inherits the first one's scale factor. Two
  # scales photographed in a row were the same picture twice before this line existed.
  $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $env:TEMP ("acwv2-" + [guid]::NewGuid().ToString('N'))
}

$proc = if ($Open -ne '') {
  # Quoted, because this repository lives under a path with a space in it and an unquoted
  # argument would arrive as two.
  Start-Process -FilePath $exe -ArgumentList ('"' + (Join-Path $root $Open) + '"') -PassThru
} else {
  Start-Process -FilePath $exe -PassThru
}
try {
  # The webview needs a moment to paint. Poll for the handle rather than guessing a duration,
  # then give the page one more beat to finish rendering before the shutter.
  $handle = [IntPtr]::Zero
  foreach ($i in 1..40) {
    Start-Sleep -Milliseconds 250
    $proc.Refresh()
    if ($proc.MainWindowHandle -ne [IntPtr]::Zero) { $handle = $proc.MainWindowHandle; break }
  }
  if ($handle -eq [IntPtr]::Zero) { throw 'the window never appeared' }
  # Windows refuses SetForegroundWindow to a process that has not been interacted with, and
  # refuses it silently. A synthetic ALT press satisfies that rule, which is the difference
  # between a keystroke reaching the window and a picture of an untouched one.
  [Win]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero)
  [Win]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)
  [void][Win]::SetForegroundWindow($handle)
  Start-Sleep -Milliseconds 1500
  # keybd_event rather than WScript.Shell's SendKeys, which needs AppActivate to have succeeded
  # and fails silently when it has not - the pictures then show an untouched window and look
  # like a broken viewer rather than a broken shutter.
  if ($Keys -ne '') {
    Start-Sleep -Milliseconds 300
    # 0x11 is Control. Held around the whole run of keys rather than per key, because that is what
    # a person's hand does and what a webview's keydown reports.
    if ($Ctrl) { [Win]::keybd_event(0x11, 0, 0, [UIntPtr]::Zero) }
    foreach ($c in $Keys.ToCharArray()) {
      # Tab has no printable character for VkKeyScan to look up, and Tab is the whole of the
      # keyboard-reachability question, so it is named directly. Write it as "`t" in -Keys.
      $vk = if ($c -eq "`t") { [byte]0x09 } else { [byte]([Win]::VkKeyScan($c) -band 0xFF) }
      [Win]::keybd_event($vk, 0, 0, [UIntPtr]::Zero)
      [Win]::keybd_event($vk, 0, 2, [UIntPtr]::Zero)
      Start-Sleep -Milliseconds 80
    }
    if ($Ctrl) { [Win]::keybd_event(0x11, 0, 2, [UIntPtr]::Zero) }
    Start-Sleep -Milliseconds $Settle
  }

  $r = New-Object Win+RECT
  [void][Win]::GetWindowRect($handle, [ref]$r)
  $w = $r.R - $r.L
  $h = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap $w, $h
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Output "wrote verification/$Name.png at ${w}x${h}"
}
finally {
  if (-not $proc.HasExited) { $proc.Kill() }
}
