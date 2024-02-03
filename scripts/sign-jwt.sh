#!/bin/bash

private_key="$1"
if [ ! -f "$private_key" ] ; then
  echo "[ERR]: $private_key does not exist"
  exit 1
fi

header="$(echo -n '{"alg":"RS256","typ":"JWT"}' | base64 | sed s/\+/-/ | sed -E s/=+$//)";
payload="$(echo -n '{"sub":"RS256inOTA","name":"John Doe"}' | base64 | sed s/\+/-/ | sed -E s/=+$//)";
signature="$(echo -n "${header}.${payload}" \
  | openssl dgst -sha256 -binary -sign "$private_key" \
  | openssl enc -base64 | tr -d '\n=' | tr -- '+/' '-_')";

jwt="${header}.${payload}.${signature}";
echo "$jwt"