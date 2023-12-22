# Unofficial Tapo REST Api

This program exposes a REST API to control your Tapo devices (light bulbs, strips, plugs, etc.)

It is based on the [unofficial Tapo API](https://crates.io/crates/tapo).

## Usage

Install this program with:

```shell
cargo install --git https://github.com/ClementNerma/tapo-rest
```

Create a JSON config file (anywhere) with the follownig structure:

```json
{
    "account": {
        "username": "<your tapo account's email>",
        "password": "<your tapo account's password>"
    },
    "devices": [
        {
            "name": "living-room-bulb",
            "device_type": "L530",
            "ip_addr": "<ip address of the device>"
        },
        {
            "name": "kitchen-bulb",
            "device_type": "L530",
            "ip_addr": "<ip address of the device>"
        }
    ]
}
```

The `name` field can be set to whatever name you want.
The `device_type` field can be any of:

* `L510`
* `L530`
* `L610`
* `L630`
* `L900`
* `L920`
* `L930`
* `P100`
* `P105`
* `P110`
* `P115`

You can then run the server with:

```shell
cargo run -- --devices-config-path <path to your json file> --port 8000 --auth-password 'potatoes'
```

This will run the server on `0.0.0.0:8000` (you can chose any port you like) and will require clients to use the `potatoes` password to log in.

**Please note though that the server is not using SSL certificates (only plain HTTP/1 and HTTP/2),** so you absolutely need to use a proxy (such as Caddy) if you don't want this secret password to appear in plain text on your network.

Before exposing the REST API, the server starts by connecting to all the devices specicified in your config file, to ensure they are reachable and caching the authentication results.

## Client usage

Clients call the `POST /login` route with a body of `{ "password": "potatoes" }`. This returns a raw string, which is the session ID.

```shell
curl -i -X POST -H 'Content-Type: application/json' --data '{ "password": "potatoes" }' http://localhost:8000/login
```

All subsequent calls to the API must include an `Authorization` header containing the session ID (`Authorization: Bearer <session ID>`). Sessions are preserved after server restart.

You can then access all other API routes which are located under `/actions` to use your device. Each route takes a `?device=<name>` query parameter to know which device you are trying to interact with. The `<name>` is the same as the one you provided in your config file.

```shell
curl -i -X GET -H 'Authorization: Bearer <your session ID>' 'http://localhost:8000/actions/l530/on?device=living-room-bulb'
```

## Query parameters

Some routes (such as `get-hourly-usage`) require timestamps. These must be provided in RFC 3339 format (e.g. `+2023-12-31`).