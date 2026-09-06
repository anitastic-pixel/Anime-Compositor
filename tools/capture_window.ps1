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
# afterwards, which is how the playback screenshot is taken.

param(
  [ValidateSet('release','debug')][string]$Build = 'release',
  [string]$Name = 'B-08_window_shell',
  [string]$Keys = '',
  [int]$Settle = 1500
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "target\$Build\anime_compositor_app.exe"
$out = Join-Path $root "verification\$Name.png"

if (-not (Test-Path $exe)) { throw "build it first: cargo build -p anime_compositor_app --$Profile" }

Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Runtime.InteropServices;
public class Win {
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
}
'@

# Without this, GetWindowRect answers in physical pixels while PowerShell thinks in scaled ones,
# and on a 150% display the shutter lands on a rectangle that is neither the window nor where it
# is. The first capture attempt photographed the desktop behind it.
[void][Win]::SetProcessDPIAware()

$proc = Start-Process -FilePath $exe -PassThru
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
  [void][Win]::SetForegroundWindow($handle)
  Start-Sleep -Milliseconds 1500
  if ($Keys -ne '') {
    $shell = New-Object -ComObject WScript.Shell
    [void]$shell.AppActivate($proc.Id)
    Start-Sleep -Milliseconds 300
    $shell.SendKeys($Keys)
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
