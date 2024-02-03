#!/bin/bash

out_dir="target/keys/$(date --iso-8601=ns | sha1sum | sed 's|  -||g;')"
mkdir -p "$out_dir"
echo "[DBG]: Created $out_dir"

private_key="${out_dir}/key.pem";
public_key="${out_dir}/key.pub.pem";

openssl genrsa -out "$private_key" 4096
openssl rsa -in "$private_key" -pubout -outform PEM -out "$public_key"

cat "$public_key"