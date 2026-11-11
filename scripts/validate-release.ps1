# PRYSM local release checks (PowerShell 7 / Windows PowerShell 5.1)
# Does not create/delete branches or worktrees; requires Git and Cargo in PATH.
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Push-Location $root
try {
  $tests = @(
    @('git', '--version'),
    @('cargo', '--version'),
    @('cargo', 'check', '--locked', '--all-targets'),
    @('cargo', 'test', '--locked', '--all-targets')
  )
  foreach ($test in $tests) {
    $exe = $test[0]
    $argsList = @($test | Select-Object -Skip 1)
    Write-Host ('> ' + $exe + ' ' + ($argsList -join ' '))
    & $exe @argsList
    if ($LASTEXITCODE -ne 0) {
      throw "$exe failed with exit code $LASTEXITCODE"
    }
  }
  Write-Host 'PRYSM source checks passed on this machine. GitHub Actions must be verified separately.'
} finally {
  Pop-Location
}
