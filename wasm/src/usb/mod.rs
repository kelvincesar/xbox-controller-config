//! Wired controller: GIP ("Gaming Input Protocol") over USB.
//!
//! Packet layout and constants follow two open-source USB drivers:
//! SDL `src/joystick/hidapi/SDL_hidapi_xboxone.c` and medusalix/xone
//! `bus/protocol.{c,h}` + `driver/common.c`.

mod adapters;
mod handlers;
mod logic;
mod model;
