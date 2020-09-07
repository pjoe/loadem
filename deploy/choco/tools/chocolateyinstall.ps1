$ErrorActionPreference = 'Stop';

$url64 = 'https://github.com/pjoe/loadem/releases/download/0.2.1/loadem-0.2.1-windows-amd64.zip'
$checksum64 = '42E4B05CA6EDAD0D874A873AEA9BFF6D0BBC123B90A1B955BF2B714D3DDE431A63CCF15CC173F478C42F5AA1529C6D3F232E96BAAD2163DDC023B0337C17CB7D'
$checksumType64 = 'sha512'
$UnzipLocation = Join-Path $env:ChocolateyInstall (Join-Path 'lib' $env:ChocolateyPackageName)
Install-ChocolateyZipPackage -PackageName $env:ChocolateyPackageName -Url64 $url64 -UnzipLocation $UnzipLocation -CheckSum64 $checksum64 -CheckSumType64 $checksumType64
Write-Output "Run 'loadem --help' to get started"