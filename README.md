# Guide light

Browser app that dims, blinks or switches off the Xbox controller's guide-button
LED. Runs entirely client-side: Rust (compiled to WebAssembly) builds the
packets, the browser's WebUSB/WebHID does the I/O, nginx only serves files.

## What works

| Link | Change LED | Notes |
|---|---|---|
| USB cable (WebUSB) | Yes | GIP command `0A 20 <seq> 03 00 <mode> <brightness>`, brightness 0–50 |
| Bluetooth (WebHID) | No | Controller only accepts rumble (report `0x03`) over Bluetooth; app shows battery + rumble test |
| Web Bluetooth | No | Chrome blocklists the HID service (`0x1812`) |

Protocol sources: SDL `src/joystick/hidapi/SDL_hidapi_xboxone.c`, medusalix/xone
`bus/protocol.c` + `driver/common.c`, atar-axis/xpadneo (Bluetooth).

Needs Chrome/Edge/Opera on a **secure origin** (HTTPS or `localhost`). The
controller forgets the setting when it powers off or reconnects.

## Layout

```
wasm/src/{usb,bluetooth}/   model · logic · adapters · handlers (wasm-bindgen boundary)
web/src/features/           connection · light · activity · help
web/src/lib/                protocol loader, shared error helpers
```

## Develop

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked   # must match wasm/Cargo.toml
cd web && npm ci && npm run wasm && npm run dev
```

Checks: `cargo test` and `cargo clippy` in `wasm/`; `npm run check`, `npm test` in `web/`.

## Deploy (Coolify)

Point Coolify at this repo with the **Dockerfile** build pack, port **80**. The
multi-stage build compiles the wasm, builds the Svelte app and serves it with
nginx. Coolify's HTTPS domain satisfies the secure-origin requirement.

```sh
docker build -t xbox-guide-light . && docker run -p 8089:80 xbox-guide-light
```
