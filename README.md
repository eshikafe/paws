# PAWS Protocol

PAWS - Protocol to Access White Space database [RFC 7545](https://datatracker.ietf.org/doc/rfc7545/)

## Setup
- Install [Rust](https://www.rust-lang.org/tools/install).
- Clone the repo: `git clone https://github.com/eshikafe/paws.git`
- Change directory: `cd paws`
- Development mode: Start the PAWS server with `cargo run --bin paws-server`
- Production mode: Run: `cargo build --release --bin paws-server` and `.\target\release\paws-server.exe`
- Access the API via URL `http://localhost:3030/v1beta/`

## How to create a binary

- `cd paws`
- `cargo build --release --bin paws-server`

## PAWS API Documentation

API version: `v1beta`.<br>
This is the API documentation for the PAWS protocol.

## Endpoints

### Version

This endpoint returns the PAWS protocol version.

| Method | Path                 |
| :----- | :------------------- |
| `GET`  | `/paws/version`      |

Sample Request

```
curl --request GET localhost:3030/v1beta/paws/version
```

Sample Response:

```
{
    "pawsVersion": "1.0"
}

```

### Initialization

This endpoint starts the PAWS `Initialization` request procedure. It allows you to send the PAWS `INIT_REQ` message. It returns a PAWS `INIT_RESP` message.

| Method | Path                 |
| :----- | :------------------- |
| `POST` | `/paws/init`         |

Sample Request

```
curl --request POST localhost:3030/v1beta/paws/init \
--header 'Content-Type: application/json' \
--data-raw '{
"jsonrpc": "2.0",
"method": "spectrum.paws.init",
"params": {
    "type": "INIT_REQ",
    "version": "1.0",
    "deviceDesc": {
      "serialNumber": "XXX",
      "fccId": "YYY",
      "rulesetIds": ["NccTvBandWhiteSpace-2019"]
     },
    "location": {
        "point": {
            "center": {"latitude": 37.0, "longitude": -101.3}
        }
    }
},
"id": "idString"
}'
```

Sample Response:

```
{
    "jsonrpc": "2.0",
    "result": {
     "type": "INIT_RESP",
     "version": "1.0",
     "rulesetInfos": [
       {
         "authority": "ng",
         "rulesetId": "NccTvBandWhiteSpace-2010",
         "maxLocationChange": 100,
         "maxPollingSecs": 86400
       }
     ]
    },
    "id": "xxxxxx"
}

```
