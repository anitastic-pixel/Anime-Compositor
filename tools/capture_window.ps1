# Starts the shell, photographs its window, and stops it again.
#
# This is the one artifact in this repository that no test can produce. Everything under
# verification/ is written by `cargo test` and CI checks that the committed copy still matches;
# a window is not a value a test can compare, so it is captured here instead and the artifact
# note beside it says so. Run from the repository root:
#
#     cargo build -p anime_compositor_app
#     powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1
#
# It writes verification/B-08_window_shell.png and prints the size it captured.

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\debug\anime_compositor_app.exe'
$out = Join-Path $root 'verification\B-08_window_shell.png'

if (-not (Test-Path $exe)) { throw "build it first: cargo build -p anime_compositor_app" }

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

  $r = New-Object Win+RECT
  [void][Win]::GetWindowRect($handle, [ref]$r)
  $w = $r.R - $r.L
  $h = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap $w, $h
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
  Write-Output "wrote verification/B-08_window_shell.png at ${w}x${h}"
}
finally {
  if (-not $proc.HasExited) { $proc.Kill() }
}
