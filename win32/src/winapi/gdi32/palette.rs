use crate::{winapi::{gdi32::{Object, HDC, HGDIOBJ}, types::WORD}, Machine};

const TRACE_CONTEXT: &'static str = "gdi32/palette";

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LOGPALETTE_HEADER {
    palVersion: WORD,
    palNumEntries: WORD,
}
unsafe impl memory::Pod for LOGPALETTE_HEADER {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
unsafe impl memory::Pod for PALETTEENTRY {}

#[derive(Debug)]
pub struct Palette(LOGPALETTE_HEADER, Vec<PALETTEENTRY>);

#[win32_derive::dllexport]
pub fn CreatePalette(machine: &mut Machine, plpal: u32) -> HGDIOBJ {
    let header = machine.mem().view::<LOGPALETTE_HEADER>(plpal).clone();

    let entries = plpal + size_of::<LOGPALETTE_HEADER>() as u32;
    let entries = machine.mem().view_n::<PALETTEENTRY>(entries, header.palNumEntries as u32).to_vec();

    machine.state.gdi32.objects.add(Object::Palette(Palette(header, entries)))
}

#[win32_derive::dllexport]
pub fn SelectPalette(machine: &mut Machine, hdc: HDC, hpal: HGDIOBJ, bForceBackground: bool) -> HGDIOBJ {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    let pal = match machine.state.gdi32.objects.get(hpal) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(pal) => pal,
    };

    match pal {
        Object::Bitmap(_) => panic!("SelectPalette called with a bitmap (use SelectObject)"),
        Object::Brush(_) => panic!("SelectPalette called with a brush (use SelectObject)"),
        Object::Palette(_) => std::mem::replace(&mut dc.palette, hpal),
        Object::Pen(_) => panic!("SelectPalette called with a pen (use SelectObject)"),
    }
}

#[win32_derive::dllexport]
pub fn RealizePalette(machine: &mut Machine, hdc: HDC) -> u32 {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return !0,
        Some(dc) => dc,
    };

    let pal = match machine.state.gdi32.objects.get(dc.palette) {
        None => return !0,
        Some(pal) => pal,
    };

    match pal {
        Object::Bitmap(_) => panic!("RealizePalette called with a bitmap"),
        Object::Brush(_) => panic!("RealizePalette called with a brush"),
        Object::Palette(palette) => palette.1.len() as u32,
        Object::Pen(_) => panic!("RealizePalette called with a pen"),
    }
}
