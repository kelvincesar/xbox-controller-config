//! Controller paired over Bluetooth, reached through WebHID.
//!
//! Over this link the controller is a plain HID gamepad: it does not accept
//! GIP packets, and neither SDL nor xpadneo knows an LED report for it. The
//! only output report is rumble (ID 0x03). Report layouts follow SDL
//! `SDL_hidapi_xboxone.c`.

mod adapters;
mod handlers;
mod logic;
mod model;
