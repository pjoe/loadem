#!/bin/bash
openssl req -newkey rsa:2048 -nodes -keyout sample-key.pem -x509 -days 3650 -out sample-cert.pem -config <(
cat <<EOF
[ req ]
default_bits        = 2048
default_keyfile     = sample-key.pem
distinguished_name  = subject
req_extensions      = extensions
x509_extensions     = extensions
string_mask         = utf8only
prompt              = no

[ subject ]
countryName = US


[ extensions ]
subjectKeyIdentifier        = hash
authorityKeyIdentifier  = keyid,issuer

basicConstraints        = CA:FALSE
keyUsage            = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage    = serverAuth
subjectAltName          = @alternate_names
nsComment           = "OpenSSL Generated Certificate"

[ alternate_names ]
DNS.1       = localhost
DNS.2       = 127.0.0.1
DNS.3       = ::1
EOF
)
