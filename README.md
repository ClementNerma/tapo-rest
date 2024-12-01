# Unofficial Tapo REST Api

This program exposes a REST API to control your Tapo devices (light bulbs, strips, plugs, etc.). It is **NOT** affiliated in any way with the Tapo or TP-Link brands and is only made as a best-effort for personal use.

It is based on the [unofficial Tapo API](https://crates.io/crates/tapo).

If you have any issue with this program, please [open an issue](https://github.com/ClementNerma/tapo-rest/issues/new)!

## Usage

You can either use a prebuilt binary from the [latest release](https://github.com/ClementNerma/tapo-rest/releases/latest) and copy it to a folder in your PATH, or use the [docker image](https://hub.docker.com/r/clementnerma/tapo-rest).

Start by creating a JSON config file (anywhere) with the following structure:

```json
{
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

* `L510`, `L520`, `L610` (light bulbs)
* `L530`, `L535`, `L630` (light bulbs with customizable colors)
* `L900` (RGB light strips)
* `L920`, `L930` (RGB light strips with individually colored segments)
* `P100`, `P105` (smart plugs)
* `P110`, `P110M`, `P115` (smart plugs with energy monitoring)

You can then run the server with:

```shell
docker run -it -v ./path-to-your-config.json:/app/devices.json clementnerma/tapo-rest \
    -p 8000:80 \
    --tapo-email '<your tapo account email address>' \
    --tapo-password '<your tapo account password>' \
    --auth-password 'potatoes'
```

You can also use environment variables, like this:

```shell
docker run -it -v ./config.json:/app/devices.json \
    -p 8000:80 \
    -e TAPO_EMAIL=... \
    -e TAPO_PASSWORD=... \
    -e AUTH_PASSWORD=... \
    clementnerma/tapo-rest
```

The prebuilt binary works the same (same flags, same environment variables).

This will run the server on port `8000` (you can chose any port you like) and will require clients to use the `potatoes` password to log in.

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

Some routes (such as `get-hourly-usage`) require timestamps. These must be provided in RFC 3339 format (e.g. `2023-12-31`).

## Session timeout

Once connected to a Tapo device, a session is maintained between the server and the device. But Tapo devices set an expiration time, which means the session will eventually expire.

If this happens, you can hit the `/refresh-session?device=...` route to refresh the session.