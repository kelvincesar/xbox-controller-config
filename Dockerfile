# syntax=docker/dockerfile:1

# 1. Rust -> WebAssembly
FROM rust:1-bookworm AS wasm
# Must match the wasm-bindgen version pinned in wasm/Cargo.toml.
ARG WASM_BINDGEN_VERSION=0.2.129
RUN rustup target add wasm32-unknown-unknown \
 && cargo install wasm-bindgen-cli --version ${WASM_BINDGEN_VERSION} --locked
WORKDIR /src
COPY wasm ./wasm
COPY scripts ./scripts
RUN mkdir -p web/src && sh scripts/build-wasm.sh

# 2. Svelte app -> static files
FROM node:22-alpine AS web
WORKDIR /src/web
COPY web/package.json web/package-lock.json ./
RUN npm ci
COPY web ./
COPY --from=wasm /src/web/src/wasm ./src/wasm
RUN npm run build

# 3. Static hosting
FROM nginx:1.27-alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=web /src/web/dist /usr/share/nginx/html
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s CMD wget -q -O /dev/null http://127.0.0.1/ || exit 1
