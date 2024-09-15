#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bitmap;
mod dc;
mod draw;
mod object;
mod palette;
mod state;
mod text;
pub use bitmap::*;
pub use dc::*;
pub use draw::*;
pub use object::*;
pub use palette::*;
pub use state::*;
pub use text::*;

pub use super::bitmap::BITMAPINFOHEADER;

pub const CLR_INVALID: u32 = 0xFFFF_FFFF;
