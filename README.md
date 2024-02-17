# Simple Auth

Simple-Auth aims to be a light-weight, easy-to-use SSO solution, compatible with the [OpenID](https://openid.net/)
specification.
Many SSO applications have a long list of features which are typically not needed for small, simple applications.
Along with these many features comes memory overhead.
Simple-Auth is designed to be run in environments with very limited processing power and memory, such as a Linux VPS
with only one CPU and one GB of memory.

The current architecture of the application will facilitate a plugin system, where daemons can extend simple-auth's
functionality.
This way, an administrator of a simple-auth instance can choose exactly how much functionality he wants and how many
computational resources each extension will add.

## Compiling

### Dependencies

`simple-auth` has the following external dependencies:
- [Sqlite3](https://www.sqlite.org/index.html)
- [libssl](https://www.libressl.org/)

For a guide on installing `libssl` so that it is compatible with the project, refer to the openssl crate 
[documentation](https://crates.io/crates/openssl).

## Running

Before running `simple-auth`, it's important to understand the configuration file.
The configuration has three main sections:
- `Server`: settings for the HTTP server.
- `Database`: settings for the database of choice.
- `Security`: a definition of the overall security strategy.

Below is an example configuration:
```json
{
  "server": {
    "domain": "localhost",
    "port": 999,
    "workers": 100
  },
  "database": {
    "Sqlite": "InMemory"
  },
  "security": {
    "jwt_signature_scheme": "PublicKey"
  },
  "log_file": "logcfg.yaml",
  "print": true,
  "banner": ".banner.txt"
}
```