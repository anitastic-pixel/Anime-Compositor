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
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
  [DllImport("user32.dll")] public static extern short VkKeyScan(char c);
  [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out POINT p);
  [DllImport("user32.dll")] public static extern void mouse_event(uint flags, uint x, uint y, uint data, UIntPtr extra);
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
}
'@

# Without this, GetWindowRect answers in physical pixels while PowerShell thinks in scaled ones,
# and on a 150% display the shutter lands on a rectangle that is neither the window nor where it
# is. The first capture attempt photographed the desktop behind it.
[void][Win]::SetProcessDPIAware()

# WebView2 reads its extra command line from this variable at startup. Set on this PowerShell
# process so the child inherits it and nothing else on the machine is affected.
#
# Software rendering, always, and only for the photograph. A GPU-composited web view draws into a
# surface that belongs to the compositor rather than to the window, and neither the screen copy
# nor PrintWindow can see it: from WebView2 151 onwards this repository's captures came back as a
# window frame around a blank white page while the page itself was fine. Checked, not guessed —
# the devtools protocol reported the body's background as rgb(20, 20, 22) and "frame 0" on screen
# in the same window that photographed white. This costs a few milliseconds of paint time in a
# script that already waits seconds, and it changes nothing about what the page renders.
#
# And occlusion detection off with it. A web view that believes nothing is looking at it stops
# painting, and then it answers a request to draw itself with the last thing it drew: the
# picture comes back showing the window as it looked at startup no matter what was typed into
# it, while the page itself has moved on. That is how a working D key photographed as a viewer
# that ignored it.
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = '--disable-gpu --disable-gpu-compositing --disable-features=CalculateNativeWinOcclusion'

if ($Scale -gt 0) {
  $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS += " --force-device-scale-factor=$Scale"
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
  # refuses it silently, so the window is *clicked* rather than asked: a click in the empty part
  # of the stage is what a person does before typing, and it does the two things asking cannot.
  # It brings the window to the front for certain, and it gives the keyboard to the web view
  # inside it rather than to the frame around it. Keys that land on the frame do nothing at all,
  # and the picture then shows a viewer that ignored the space bar.
  [void][Win]::ShowWindow($handle, 9)   # SW_RESTORE
  $spot = New-Object Win+RECT
  [void][Win]::GetWindowRect($handle, [ref]$spot)
  $was = New-Object Win+POINT
  [void][Win]::GetCursorPos([ref]$was)
  foreach ($i in 1..20) {
    [void][Win]::SetCursorPos($spot.L + ($spot.R - $spot.L) / 2, $spot.T + ($spot.B - $spot.T) / 4)
    [Win]::mouse_event(0x02, 0, 0, 0, [UIntPtr]::Zero)   # left down
    [Win]::mouse_event(0x04, 0, 0, 0, [UIntPtr]::Zero)   # left up
    [void][Win]::SetForegroundWindow($handle)
    Start-Sleep -Milliseconds 250
    if ([Win]::GetForegroundWindow() -eq $handle) { break }
  }
  [void][Win]::SetCursorPos($was.X, $was.Y)
  # Checked, not hoped for: a picture of a window that never got the keyboard is worse than no
  # picture, because it looks like a finding.
  if ([Win]::GetForegroundWindow() -ne $handle) { throw 'the window would not come to the front' }
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
  # PrintWindow with PW_RENDERFULLCONTENT (2), and not a copy off the screen: it asks the window
  # to draw itself, which includes the web view's own child window, and it does not depend on
  # what happens to be in front of the window at the instant of the shutter.
  $hdc = $g.GetHdc()
  $ok = [Win]::PrintWindow($handle, $hdc, 2)
  $g.ReleaseHdc($hdc)
  if (-not $ok) { throw 'the window refused to draw itself' }
  $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Output "wrote verification/$Name.png at ${w}x${h}"
}
finally {
  if (-not $proc.HasExited) { $proc.Kill() }
}
