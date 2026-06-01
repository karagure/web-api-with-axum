# tp-wik-dps-01

A minimal HTTP API built with [Axum](https://github.com/tokio-rs/axum).

## Endpoints

| Method | Path     | Response                                            |
| ------ | -------- | --------------------------------------------------- |
| GET    | `/ping`  | `200 OK` with JSON body `{"status":"ok"}`           |
| any    | anything | `404 Not Found`, headers only (no body)             |

A wrong method on a known route (e.g. `POST /ping`) also returns `404`.

## Configuration

The server reads the listening port from the `PORT` environment variable,
loaded from a `.env` file. It defaults to `3000` when unset.

## Documentation

The generated API documentation is published via GitHub Pages:
<https://karagure.github.io/web-api-with-axum/tp_wik_dps_01/>

The HTML lives in [`docs/`](docs/) and is served once GitHub Pages is enabled
(Settings → Pages → Source: branch `main`, folder `/docs`).
