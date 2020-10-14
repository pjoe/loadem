import-module au

$domain = 'https://github.com'
$releases = "$domain/pjoe/loadem/releases/latest"

function global:au_SearchReplace {
  @{
    ".\tools\chocolateyInstall.ps1" = @{
      "(url64\s*=\s*).*$" = "`$1'$($Latest.URL64)'"
      "(checksum64\s*=\s*).*$" = "`$1'$($Latest.Checksum64)'"
      "(checksumType64\s*=\s*).*$" = "`$1'$($Latest.ChecksumType64)'"
      #"(?i)(^\s*url64\s*=\s*)('.*')"          = "`$1'$($Latest.URL64)'"
      # "(?i)(^\s*checksum64\s*=\s*)('.*')"     = "`$1'$($Latest.Checksum64)'"
      # "(?i)(^\s*checksumType64\s*=\s*)('.*')" = "`$1'$($Latest.ChecksumType64)'"
    }
    # ".\loadem.nuspec" = @{
    # }
  }
}

function global:au_GetLatest {
  $download_page = Invoke-WebRequest -UseBasicParsing -Uri $releases

  $re = 'windows-amd64\.zip$'
  $url = $download_page.links | Where-Object href -match $re | Select-Object -First 1 -expand href

  $version = ($url -split '/' | Select-Object -Last 1 -Skip 1)

  write-host "version: $version"
  @{
    URL64          = $domain + $url
    Version        = $version
    ChecksumType64 = 'sha512'
  }
}

Update-Package -ChecksumFor 64
Push-Package