# PAWS Protocol

PAWS - Protocol to Access White Space database [RFC 7545]

## PAWS API

API version: v1beta.

The API allows you to interact with the PAWS protocol.

Start the PAWS server with `cargo run --bin server` and then access the API with URL `http://localhost:3030/v1beta/paws`

## Endpoints

### Version

GET `/version`

Returns the PAWS protocol version.

Example

```
curl --request GET localhost:3030/v1beta/paws/version
```

Response:

```
{
    "pawsVersion": "1.0"
}

```

### Init

POST `/init`

Starts the PAWS `Initialization` request procedure. It allows you to send the PAWS `INIT_REQ` message. It returns a PAWS `INIT_RESP` message.

Example

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
    },
"id": "idString"
}'
```

Response:

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
