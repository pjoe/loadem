$ErrorActionPreference = 'Stop';

$url64 = 'https://github.com/pjoe/loadem/releases/download/0.2.2/loadem-0.2.2-windows-amd64.zip'
$checksum64 = '4ED5713A5DF1104A257DDF3D1360A568572A583C089CFFAACACDE4A90D8B1656C200F0187BE6D105232911E294CEC32FC471FDEA7489E88D28FADD1614E84FC6'
$checksumType64 = 'sha512'
$UnzipLocation = Join-Path $env:ChocolateyInstall (Join-Path 'lib' $env:ChocolateyPackageName)
Install-ChocolateyZipPackage -PackageName $env:ChocolateyPackageName -Url64 $url64 -UnzipLocation $UnzipLocation -CheckSum64 $checksum64 -CheckSumType64 $checksumType64
Write-Output "Run 'loadem --help' to get started"