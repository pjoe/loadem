$ErrorActionPreference = 'Stop';

$url64 = 'https://github.com/pjoe/loadem/releases/download/0.2.2/loadem-0.2.2-windows-amd64.zip'
$checksum64 = '4ED5713A5DF1104A257DDF3D1360A568572A583C089CFFAACACDE4A90D8B1656C200F0187BE6D105232911E294CEC32FC471FDEA7489E88D28FADD1614E84FC6'
$checksumType64 = 'sha512'
$UnzipLocation = Join-Path $env:ChocolateyInstall (Join-Path 'lib' $env:ChocolateyPackageName)

# need to add package to PATH instead of using shim, in order to have proper Ctrl-C handling
# this ensures loadem can output stats to stdout when a test is quit using Ctrl-C

# make sure dir exists
mkdir $UnzipLocation -ErrorAction SilentlyContinue
# ignore exe to avoid shim
New-Item "$UnzipLocation\loadem.exe.ignore" -type file -force | Out-Null
# delete old shim if exists
Uninstall-BinFile 'loadem'
# install
Install-ChocolateyZipPackage -PackageName $env:ChocolateyPackageName -Url64 $url64 -UnzipLocation $UnzipLocation -CheckSum64 $checksum64 -CheckSumType64 $checksumType64
# add to path
Install-ChocolateyPath $UnzipLocation -PathType 'Machine'

Write-Output "Run 'loadem --help' to get started"
