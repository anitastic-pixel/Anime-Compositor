# Stages a portable copy of the application, with the papers that have to travel with it.
#
#     cargo build -p anime_compositor_app --release
#     powershell -ExecutionPolicy Bypass -File tools/package.ps1
#
# It writes target/package/AnimeCompositor-<version>/ and a .zip of the same, and prints what it
# put there. Nothing under verification/ or Licenses/ is modified.
#
# This is not an installer. `app/tauri.conf.json` leaves `bundle.active` false and no MSI or NSIS
# package is produced; docs/SUPPORTED_ENVELOPE.md says why and what that costs the person running
# it. What this does produce is a folder that can be copied to a machine and run, which is what
# version 0.1 needs, and which carries the licence texts document 10 requires a distribution to
# carry.

param([ValidateSet('release','debug')][string]$Build = 'release')

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "target\$Build\anime_compositor_app.exe"
if (-not (Test-Path $exe)) { throw "build it first: cargo build -p anime_compositor_app --$Build" }

# The version the application reports, taken from the manifest rather than typed here, so a
# package can never be named after a version that was not built.
$version = (Select-String -Path (Join-Path $root 'app\tauri.conf.json') -Pattern '"version": "([^"]+)"').Matches[0].Groups[1].Value
$stage = Join-Path $root "target\package\AnimeCompositor-$version"

if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
New-Item -ItemType Directory -Force -Path $stage | Out-Null

# .NET rather than Copy-Item, which some sandboxes refuse even for a source and a destination that
# are both inside this repository.
function Put([string]$from, [string]$to) {
  $target = Join-Path $stage $to
  New-Item -ItemType Directory -Force -Path (Split-Path -Parent $target) | Out-Null
  [System.IO.File]::Copy($from, $target, $true)
}

Put $exe 'Anime Compositor.exe'
Put (Join-Path $root 'LICENSE-MIT') 'LICENSE-MIT'
Put (Join-Path $root 'LICENSE-APACHE') 'LICENSE-APACHE'
Put (Join-Path $root 'docs\DEPENDENCIES.md') 'DEPENDENCIES.md'
Put (Join-Path $root 'docs\SUPPORTED_ENVELOPE.md') 'READ ME FIRST.md'

# Every dependency's licence text, whole. Document 10 asks that a distribution carry these, and a
# summary of them is not one of them.
# Each crate has a directory of its own and several of them use the same file names, so the path
# below Licenses/ is kept rather than the file name. Flattening this silently loses all but one
# copy of every LICENSE-MIT in the archive.
$licenses = Join-Path $root 'Licenses'
foreach ($file in Get-ChildItem $licenses -File -Recurse) {
  Put $file.FullName (Join-Path 'Licenses' $file.FullName.Substring($licenses.Length + 1))
}

$zip = "$stage.zip"
if (Test-Path $zip) { Remove-Item -Force $zip }
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip

$size = [math]::Round((Get-Item $zip).Length / 1MB, 1)
Write-Output "staged $stage"
Write-Output ("files: " + (Get-ChildItem $stage -Recurse -File).Count)
Write-Output "zipped $zip, $size MB"
