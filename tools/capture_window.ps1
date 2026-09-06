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
# afterwards, which is how the playback screenshot is taken; -Open starts the shell on a project
# file, which is the same path a dropped file takes and the only one a script can drive.

param(
  [ValidateSet('release','debug')][string]$Build = 'release',
  [string]$Name = 'B-08_window_shell',
  [string]$Keys = '',
  [int]$Settle = 1500,
  [string]$Open = ''
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
    foreach ($c in $Keys.ToCharArray()) {
      $vk = [byte]([Win]::VkKeyScan($c) -band 0xFF)
      [Win]::keybd_event($vk, 0, 0, [UIntPtr]::Zero)
      [Win]::keybd_event($vk, 0, 2, [UIntPtr]::Zero)
      Start-Sleep -Milliseconds 80
    }
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
