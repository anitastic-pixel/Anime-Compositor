# Rebuilds the shell whenever its source changes, and reopens it on the project it had open.
#
# For the owner's sittings with the agent: leave this running in its own terminal, and every
# change the agent saves under app/ or src/ turns into a fresh window a moment later, without
# anybody having to close the old one and start the new one by hand. Run from the repository root:
#
#     powershell -ExecutionPolicy Bypass -File tools/watch_rebuild.ps1
#
# It waits until the files have been quiet for -Quiet seconds (a batch of edits lands as several
# saves), asks the window to close (so that the unsaved-work recovery the shell already has can
# do its job if the close is refused, it is stopped after -Grace seconds anyway), builds, and
# starts the window again on -Open - by default the most recent project in the shell's own
# recent list. A build that fails prints the compiler's words and starts the old window again,
# so there is always a window to look at. Ctrl+C stops the watch; the window stays open.

param(
  [string]$Open = '',
  [int]$Quiet = 4,
  [int]$Grace = 5
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\release\anime_compositor_app.exe'
$watched = @('app\src', 'app\ui', 'src', 'app\Cargo.toml', 'Cargo.toml') | ForEach-Object { Join-Path $root $_ }

function Stamp {
  # One string for the state of every watched file: its path and last write time. Comparing two
  # of these is the whole change detector; a few hundred files polled once a second is nothing.
  ($watched | ForEach-Object {
    if (Test-Path $_ -PathType Container) { Get-ChildItem $_ -Recurse -File } else { Get-Item $_ }
  } | ForEach-Object { "$($_.FullName)|$($_.LastWriteTimeUtc.Ticks)" }) -join "`n"
}

function Project {
  if ($Open) { return $Open }
  $recent = Join-Path $env:APPDATA 'dev.anitastic.anime-compositor\recent.txt'
  if (Test-Path $recent) {
    $first = Get-Content $recent -TotalCount 1
    if ($first -and (Test-Path $first)) { return $first }
  }
  return ''
}

function Start-Shell {
  $project = Project
  if ($project) { Start-Process $exe -ArgumentList "`"$project`"" } else { Start-Process $exe }
  Write-Host ("{0}  opened {1}" -f (Get-Date -Format 'HH:mm:ss'), $(if ($project) { $project } else { 'the shell' }))
}

function Stop-Shell {
  $running = Get-Process anime_compositor_app -ErrorAction SilentlyContinue
  if (-not $running) { return }
  $running | ForEach-Object { $null = $_.CloseMainWindow() }
  if (-not ($running | Wait-Process -Timeout $Grace -ErrorAction SilentlyContinue)) {
    Get-Process anime_compositor_app -ErrorAction SilentlyContinue | Stop-Process -Force
  }
}

function Rebuild {
  Stop-Shell
  Write-Host ("{0}  building..." -f (Get-Date -Format 'HH:mm:ss'))
  Push-Location $root
  # Through cmd, because cargo talks on stderr and Windows PowerShell turns a redirected stderr
  # line into an error record, which under 'Stop' ended the whole watch on the first build.
  try { & cmd /c 'cargo build --release -p anime_compositor_app 2>&1' | Where-Object { $_ -notmatch '^\s*(Compiling|Finished)' } | Write-Host }
  finally { Pop-Location }
  if ($LASTEXITCODE -ne 0) { Write-Host '  the build failed; the last window that built is opened instead' -ForegroundColor Yellow }
  Start-Shell
}

Write-Host "watching app/ and src/ for changes; Ctrl+C to stop"
if (-not (Get-Process anime_compositor_app -ErrorAction SilentlyContinue)) {
  if (Test-Path $exe) { Start-Shell } else { Rebuild }
}
$last = Stamp
$changedAt = $null
while ($true) {
  Start-Sleep -Seconds 1
  $now = Stamp
  if ($now -ne $last) { $last = $now; $changedAt = Get-Date; continue }
  if ($changedAt -and ((Get-Date) - $changedAt).TotalSeconds -ge $Quiet) {
    $changedAt = $null
    try { Rebuild } catch { Write-Host "  $_" -ForegroundColor Red }
    $last = Stamp
  }
}
