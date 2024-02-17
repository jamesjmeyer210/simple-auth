/// The [JsonWebKeySet] should be exposed at https://{domain}/.well-known/jwks.json
/// An [JsonWebKeySet] example instance in json:
/// ```json
/// {
///     "keys": [
///     {
///         "alg": "RS256",
///         "kty": "RSA",
///         "use": "sig",
///         "x5c": [
///             "MIIC+DCCAeCgAwIBAgI...ashGkkgmo="
///         ],
///         "n": "yeNlzlub94YgerT..._mW3HoBdjQ",
///         "e": "AQAB",
///         "kid": "NjVBRjY5MDlCMUIwNzU4RTA2QzZFMDQ4QzQ2MDAyQjVDNjk1RTM2Qg",
///         "x5t": "NjVBRjY5MDlCMUIwNzU4RTA2QzZFMDQ4QzQ2MDAyQjVDNjk1RTM2Qg"
///     }
/// ]}
/// ```
pub struct JsonWebKeySet {
    keys: Vec<JsonWebKey>
}

/// A full explanation for this type may be found in the [auth0 documentation](https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-key-set-properties).
pub struct JsonWebKey {
    /// The cryptographic algorithm used with the key.
    /// Example: `"RS256"`
    pub alg: String,
    /// The family of crypto algorithms use with the key
    pub kty: String,
    /// How the key is meant to be used.
    /// For example, `"sig"` for signature.
    pub _use: String,
    /// The x.509 certificate chain.
    /// The first entry in the array is the certificate to use for token verification.
    /// The other certificates can be used to verify the first certificate.
    pub x5c: Vec<String>,
    /// The modulus for the RSA public key
    pub n: String,
    /// The exponent for the RSA public key
    pub e: String,
    /// The unique identifier for the key
    pub kid: String,
    /// The SHA1 thumbprint for the x.509 cert
    pub x5t: String
}