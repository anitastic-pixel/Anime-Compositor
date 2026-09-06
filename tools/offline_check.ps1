# Watches the running application for network connections. T-10, the half a test cannot do.
#
# `tests/b11_offline_record.rs` reads the build and says what could reach a network. This runs the
# program and says what did. It starts the shell on a project, watches every process in its tree -
# the shell itself and the web view processes Windows starts underneath it - and records every TCP
# connection any of them holds, once a second, for the duration.
#
#     cargo build -p anime_compositor_app --release
#     powershell -ExecutionPolicy Bypass -File tools/offline_check.ps1 -Open "target\shot\my_shot.json"
#
# What it cannot do is disconnect the machine. Run it with the cable out or the adapter disabled to
# check the other half of R-11, that the program still works when there is nothing to talk to; the
# result of this script is only "it did not try". Both halves are worth having and neither is the
# other.

param(
  [ValidateSet('release','debug')][string]$Build = 'release',
  [string]$Open = '',
  [int]$Seconds = 20
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "target\$Build\anime_compositor_app.exe"
if (-not (Test-Path $exe)) { throw "build it first: cargo build -p anime_compositor_app --$Build" }

$proc = if ($Open -ne '') {
  Start-Process -FilePath $exe -ArgumentList ('"' + (Join-Path $root $Open) + '"') -PassThru
} else {
  Start-Process -FilePath $exe -PassThru
}

# Every descendant of a process, by walking the parent links Windows keeps. The web view runs in
# its own processes, so watching only the one we started would watch the wrong thing.
function Get-Tree([int]$id) {
  $all = Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId, Name
  $out = @{}
  $queue = New-Object System.Collections.Queue
  $queue.Enqueue($id)
  while ($queue.Count -gt 0) {
    $current = $queue.Dequeue()
    if ($out.ContainsKey($current)) { continue }
    $row = $all | Where-Object { $_.ProcessId -eq $current } | Select-Object -First 1
    if ($row) { $out[$current] = $row.Name }
    foreach ($child in $all | Where-Object { $_.ParentProcessId -eq $current }) {
      $queue.Enqueue([int]$child.ProcessId)
    }
  }
  $out
}

try {
  $seen = @{}
  $connections = New-Object System.Collections.Generic.List[string]
  foreach ($i in 1..$Seconds) {
    Start-Sleep -Seconds 1
    if ($proc.HasExited) { break }
    $tree = Get-Tree $proc.Id
    foreach ($id in $tree.Keys) { $seen[$id] = $tree[$id] }
    foreach ($c in Get-NetTCPConnection -ErrorAction SilentlyContinue) {
      if ($tree.ContainsKey([int]$c.OwningProcess)) {
        $line = "{0} {1}:{2} -> {3}:{4} {5}" -f $tree[[int]$c.OwningProcess], `
          $c.LocalAddress, $c.LocalPort, $c.RemoteAddress, $c.RemotePort, $c.State
        if (-not $connections.Contains($line)) { $connections.Add($line) }
      }
    }
  }

  Write-Output "watched for $Seconds seconds"
  Write-Output ("processes in the tree: " + (($seen.Values | Sort-Object -Unique) -join ', '))
  # A connection whose remote address is this machine is not the network. Everything else is.
  $offMachine = $connections | Where-Object { $_ -notmatch '-> (127\.0\.0\.1|::1|0\.0\.0\.0|::):' }
  Write-Output ("tcp connections held, in total: " + $connections.Count)
  Write-Output ("of those, to somewhere off this machine: " + $offMachine.Count)
  foreach ($line in $connections) { Write-Output "  $line" }
}
finally {
  if (-not $proc.HasExited) { $proc.Kill() }
}
