## PAWS protocol

PAWS - Protocol to Access White Space database [RFC 7545]

# PAWS API

The API allows you to interact with the PAWS protocol.

Start the PAWS server with `cargo run --bin server` to access the API.
API is available at http://localhost:3030/v1beta/paws/

## Endpoints

### Version

GET `/version`

Returns the PAWS protocol version.

Example

```
curl --request GET localhost:3030/v1beta/paws/version

{
    "PAWS Version": "1.0"
}

```
