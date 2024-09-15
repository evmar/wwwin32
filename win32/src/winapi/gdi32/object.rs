use super::{Brush, DCTarget, Palette, Pen, BITMAP, COLORREF, HDC};
use crate::{
    winapi::{
        bitmap::{Bitmap, BitmapMono, BitmapRGBA32},
        types::HANDLE,
    },
    Machine,
};

const TRACE_CONTEXT: &'static str = "gdi32/object";

#[derive(Debug)]
pub enum BitmapType {
    RGBA32(BitmapRGBA32),
    Mono(BitmapMono),
}

impl BitmapType {
    pub fn inner(&self) -> &dyn Bitmap {
        match self {
            BitmapType::RGBA32(b) => b,
            BitmapType::Mono(b) => b,
        }
    }
}

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(BitmapType),
    Palette(Palette),
    Pen(Pen),
}

pub type HGDIOBJ = HANDLE<Object>;
impl HGDIOBJ {
    /// Some Windows APIs use low values of GDI objects as known system constants,
    /// so start the handles higher.
    pub fn lowest_value() -> u32 {
        0x100
    }
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum GetStockObjectArg {
    WHITE_BRUSH = 0,
    LTGRAY_BRUSH = 1,
    GRAY_BRUSH = 2,
    DKGRAY_BRUSH = 3,
    BLACK_BRUSH = 4,
    NULL_BRUSH = 5,
    OEM_FIXED_FONT = 10,
}

#[win32_derive::dllexport]
pub fn GetStockObject(machine: &mut Machine, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    match i.unwrap() {
        GetStockObjectArg::WHITE_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF((0xff, 0xff, 0xff))),
        })),
        GetStockObjectArg::LTGRAY_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF((0xc0, 0xc0, 0xc0))),
        })),
        GetStockObjectArg::BLACK_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: Some(COLORREF((0x00, 0x00, 0x00))),
        })),
        GetStockObjectArg::NULL_BRUSH => machine
            .state
            .gdi32
            .objects
            .add(Object::Brush(Brush { color: None })),
        GetStockObjectArg::OEM_FIXED_FONT => {
            log::error!("returning null stock object");
            HGDIOBJ::null()
        }
        _ => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: HDC, hGdiObj: HGDIOBJ) -> HGDIOBJ {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    let obj = match machine.state.gdi32.objects.get(hGdiObj) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(obj) => obj,
    };
    match obj {
        Object::Bitmap(_) => match dc.target {
            DCTarget::Memory(prev) => {
                dc.target = DCTarget::Memory(hGdiObj);
                prev
            }
            DCTarget::Window(_) => todo!(),
            DCTarget::DirectDrawSurface(_) => todo!(),
        },
        Object::Brush(_) => std::mem::replace(&mut dc.brush, hGdiObj),
        Object::Palette(_) => panic!("SelectObject called with a palette (use SelectPalette)"),
        Object::Pen(_) => std::mem::replace(&mut dc.pen, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: HGDIOBJ, bytes: u32, out: u32) -> u32 {
    let obj = match machine.state.gdi32.objects.get(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };

    match obj {
        Object::Brush(_) => todo!(),
        Object::Bitmap(bitmap) => {
            assert_eq!(bytes as usize, std::mem::size_of::<BITMAP>());
            let out = machine.mem().view_mut::<BITMAP>(out);
            let bitmap = bitmap.inner();
            *out = BITMAP {
                bmType: 0,
                bmWidth: bitmap.width(),
                bmHeight: bitmap.height(),
                bmWidthBytes: 0,
                bmPlanes: 0,
                bmBitsPixel: 0,
                bmBits: 0,
            };
            bytes
        }
        Object::Palette(_) => todo!(),
        Object::Pen(_) => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn DeleteObject(_machine: &mut Machine, handle: HGDIOBJ) -> bool {
    // TODO: leak
    true
}
