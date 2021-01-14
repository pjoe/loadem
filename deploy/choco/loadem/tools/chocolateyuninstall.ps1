$ErrorActionPreference = 'Stop'

# remove from path on uninstall
$PackageLocation = Join-Path $env:ChocolateyInstall (Join-Path 'lib' $env:ChocolateyPackageName)

# get path
$path = [System.Environment]::GetEnvironmentVariable('PATH', 'Machine')
# remove unwanted elements
$path = ($path.Split(';') | Where-Object { $_ -ne $PackageLocation }) -join ';'
# set it
[System.Environment]::SetEnvironmentVariable('PATH', $path, 'Machine')