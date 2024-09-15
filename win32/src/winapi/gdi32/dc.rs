use super::{BitmapType, Object, HGDIOBJ, R2};
use crate::winapi::types::POINT;
use crate::{
    machine::Machine,
    winapi::{
        bitmap::{BitmapMono, PixelData},
        types::{HANDLE, HWND},
    },
};

const TRACE_CONTEXT: &'static str = "gdi32/dc";

pub type HDC = HANDLE<DC>;

/// Target device for a DC.
#[derive(Debug)]
pub enum DCTarget {
    Memory(HGDIOBJ), // aka Bitmap
    Window(HWND),
    DirectDrawSurface(u32),
}

#[derive(Debug)]
pub struct DC {
    // TODO: it's unclear to me what the representation of a DC ought to be.
    // DirectDraw can also create a DC, and DirectDraw (as a DLL that came
    // later) can't retrofit the DC type with a DirectDraw field.
    // Wine appears to use a vtable (for generic behavior).
    pub target: DCTarget,

    pub r2: R2,
    pub x: u32,
    pub y: u32,

    // The SelectObject() API sets a drawing-related field on the DC and returns the
    // previously selected object of a given type, which means we need a storage field
    // per object type.
    pub brush: HGDIOBJ,
    pub pen: HGDIOBJ,
    pub palette: HGDIOBJ,
}

impl DC {
    pub fn new(target: DCTarget) -> Self {
        DC {
            target,
            r2: R2::default(),
            x: 0,
            y: 0,
            brush: Default::default(),
            pen: Default::default(),
            palette: Default::default(),
        }
    }

    pub fn new_memory(machine: &mut Machine) -> Self {
        // MSDN says: "When a memory device context is created, it initially has a 1-by-1 monochrome bitmap selected into it."
        // SkiFree depends on this!
        let bitmap = BitmapMono {
            width: 1,
            height: 1,
            pixels: PixelData::Ptr(0, 0),
        };
        let hobj = machine
            .state
            .gdi32
            .objects
            .add(Object::Bitmap(BitmapType::Mono(bitmap)));
        Self::new(DCTarget::Memory(hobj))
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(machine: &mut Machine, hdc: HDC) -> HDC {
    let dc = DC::new_memory(machine);
    let handle = machine.state.gdi32.dcs.add(dc);
    handle
}

#[win32_derive::dllexport]
pub fn DeleteDC(_machine: &mut Machine, hdc: u32) -> u32 {
    log::warn!("todo: DeleteDC({hdc:x})");
    0 // fail
}

#[derive(Debug, win32_derive::TryFromEnum)]
#[repr(u32)]
pub enum GetDeviceCapsArg {
    DRIVERVERSION = 0,
    TECHNOLOGY = 2,
    HORZSIZE = 4,
    VERTSIZE = 6,
    HORZRES = 8,
    VERTRES = 10,
    BITSPIXEL = 12,
    PLANES = 14,
    NUMBRUSHES = 16,
    NUMPENS = 18,
    NUMMARKERS = 20,
    NUMFONTS = 22,
    NUMCOLORS = 24,
    PDEVICESIZE = 26,
    CURVECAPS = 28,
    LINECAPS = 30,
    POLYGONALCAPS = 32,
    TEXTCAPS = 34,
    CLIPCAPS = 36,
    RASTERCAPS = 38,
    ASPECTX = 40,
    ASPECTY = 42,
    ASPECTXY = 44,
    LOGPIXELSX = 88,
    LOGPIXELSY = 90,
    SIZEPALETTE = 104,
    NUMRESERVED = 106,
    COLORRES = 108,
    PHYSICALWIDTH = 110,
    PHYSICALHEIGHT = 111,
    PHYSICALOFFSETX = 112,
    PHYSICALOFFSETY = 113,
    SCALINGFACTORX = 114,
    SCALINGFACTORY = 115,
    VREFRESH = 116,
    DESKTOPVERTRES = 117,
    DESKTOPHORZRES = 118,
    BLTALIGNMENT = 119,
}

#[win32_derive::dllexport]
pub fn GetDeviceCaps(
    _machine: &mut Machine,
    hdc: HDC,
    index: Result<GetDeviceCapsArg, u32>,
) -> u32 {
    match index.unwrap() {
        GetDeviceCapsArg::NUMCOLORS => -1i32 as u32, // true color
        GetDeviceCapsArg::HORZRES => 640,
        GetDeviceCapsArg::VERTRES => 480,
        GetDeviceCapsArg::RASTERCAPS => 0x100, // RC_PALETTE
        index => unimplemented!("GetDeviceCaps index={:?}", index),
    }
}

#[win32_derive::dllexport]
pub fn GetLayout(_machine: &mut Machine, hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn GetDCOrgEx(machine: &mut Machine, hdc: HDC, lpPoint: Option<&mut POINT>) -> bool {
    let dc = machine.state.gdi32.dcs.get_mut(hdc).unwrap();
    if let Some(lpPoint) = lpPoint {
        lpPoint.x = dc.x;
        lpPoint.y = dc.y;
        return true;
    }
    false
}
