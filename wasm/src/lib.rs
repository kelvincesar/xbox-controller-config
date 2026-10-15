//! Xbox controller protocol, compiled to WebAssembly.
//!
//! The browser owns all I/O (WebUSB / WebHID). This crate decides which bytes
//! to send and how to read the ones that come back. Each feature is split by
//! role: `model` (domain types), `logic` (pure decisions), `adapters`
//! (wire <-> model translation) and `handlers` (the wasm-bindgen boundary).

mod bluetooth;
mod usb;
