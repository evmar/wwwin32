#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[doc = r" Generated code, do not edit."]
use crate::shims::Shim;
pub struct BuiltinDLL {
    pub file_name: &'static str,
    #[doc = r" The xth function in the DLL represents a call to shims[x]."]
    pub shims: &'static [Shim],
    #[doc = r" Raw bytes of generated .dll."]
    pub raw: &'static [u8],
}
pub mod advapi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::advapi32::*;
        pub unsafe fn RegCloseKey(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            winapi::advapi32::RegCloseKey(machine, hKey).to_raw()
        }
        pub unsafe fn RegCreateKeyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&str>>::from_stack(mem, esp + 8u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::advapi32::RegCreateKeyA(machine, hKey, lpSubKey, phkResult).to_raw()
        }
        pub unsafe fn RegCreateKeyExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpSubKey = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let lpClass = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            let dwOptions = <u32>::from_stack(mem, esp + 20u32);
            let samDesired = <u32>::from_stack(mem, esp + 24u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 28u32);
            let phkResult = <Option<&mut u32>>::from_stack(mem, esp + 32u32);
            let lpdwDisposition = <Option<&mut u32>>::from_stack(mem, esp + 36u32);
            winapi::advapi32::RegCreateKeyExW(
                machine,
                hKey,
                lpSubKey,
                Reserved,
                lpClass,
                dwOptions,
                samDesired,
                lpSecurityAttributes,
                phkResult,
                lpdwDisposition,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegQueryValueExA(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegQueryValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpReserved = <u32>::from_stack(mem, esp + 12u32);
            let lpType = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let lpcbData = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegQueryValueExW(
                machine,
                hKey,
                lpValueName,
                lpReserved,
                lpType,
                lpData,
                lpcbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&str>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let dwType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegSetValueExA(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            )
            .to_raw()
        }
        pub unsafe fn RegSetValueExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hKey = <HKEY>::from_stack(mem, esp + 4u32);
            let lpValueName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let Reserved = <u32>::from_stack(mem, esp + 12u32);
            let dwType = <u32>::from_stack(mem, esp + 16u32);
            let lpData = <u32>::from_stack(mem, esp + 20u32);
            let cbData = <u32>::from_stack(mem, esp + 24u32);
            winapi::advapi32::RegSetValueExW(
                machine,
                hKey,
                lpValueName,
                Reserved,
                dwType,
                lpData,
                cbData,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const RegCloseKey: Shim = Shim {
            name: "RegCloseKey",
            func: impls::RegCloseKey,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegCreateKeyA: Shim = Shim {
            name: "RegCreateKeyA",
            func: impls::RegCreateKeyA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const RegCreateKeyExW: Shim = Shim {
            name: "RegCreateKeyExW",
            func: impls::RegCreateKeyExW,
            stack_consumed: 36u32,
            is_async: false,
        };
        pub const RegQueryValueExA: Shim = Shim {
            name: "RegQueryValueExA",
            func: impls::RegQueryValueExA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const RegQueryValueExW: Shim = Shim {
            name: "RegQueryValueExW",
            func: impls::RegQueryValueExW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const RegSetValueExA: Shim = Shim {
            name: "RegSetValueExA",
            func: impls::RegSetValueExA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const RegSetValueExW: Shim = Shim {
            name: "RegSetValueExW",
            func: impls::RegSetValueExW,
            stack_consumed: 24u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 7usize] = [
        shims::RegCloseKey,
        shims::RegCreateKeyA,
        shims::RegCreateKeyExW,
        shims::RegQueryValueExA,
        shims::RegQueryValueExW,
        shims::RegSetValueExA,
        shims::RegSetValueExW,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "advapi32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/advapi32.dll"),
    };
}
pub mod bass {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::bass::*;
        pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_ChannelGetPosition(machine, mode).to_raw()
        }
        pub unsafe fn BASS_Init(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4).to_raw()
        }
        pub unsafe fn BASS_MusicLoad(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            let arg3 = <u32>::from_stack(mem, esp + 12u32);
            let arg4 = <u32>::from_stack(mem, esp + 16u32);
            let arg5 = <u32>::from_stack(mem, esp + 20u32);
            winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5).to_raw()
        }
        pub unsafe fn BASS_MusicPlay(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            winapi::bass::BASS_MusicPlay(machine, arg1).to_raw()
        }
        pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let arg1 = <u32>::from_stack(mem, esp + 4u32);
            let arg2 = <u32>::from_stack(mem, esp + 8u32);
            winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2).to_raw()
        }
        pub unsafe fn BASS_Start(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::bass::BASS_Start(machine).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const BASS_ChannelGetPosition: Shim = Shim {
            name: "BASS_ChannelGetPosition",
            func: impls::BASS_ChannelGetPosition,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const BASS_Init: Shim = Shim {
            name: "BASS_Init",
            func: impls::BASS_Init,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const BASS_MusicLoad: Shim = Shim {
            name: "BASS_MusicLoad",
            func: impls::BASS_MusicLoad,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const BASS_MusicPlay: Shim = Shim {
            name: "BASS_MusicPlay",
            func: impls::BASS_MusicPlay,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const BASS_MusicSetPositionScaler: Shim = Shim {
            name: "BASS_MusicSetPositionScaler",
            func: impls::BASS_MusicSetPositionScaler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const BASS_Start: Shim = Shim {
            name: "BASS_Start",
            func: impls::BASS_Start,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 6usize] = [
        shims::BASS_ChannelGetPosition,
        shims::BASS_Init,
        shims::BASS_MusicLoad,
        shims::BASS_MusicPlay,
        shims::BASS_MusicSetPositionScaler,
        shims::BASS_Start,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/bass.dll"),
    };
}
pub mod ddraw {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ddraw::*;
        pub unsafe fn DirectDrawCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreate(machine, lpGuid, lplpDD, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectDrawCreateClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lplpDDClipper = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::DirectDrawCreateClipper(machine, dwFlags, lplpDDClipper, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn DirectDrawCreateEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let lplpDD = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let iid = <Option<&GUID>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::DirectDrawCreateEx(machine, lpGuid, lplpDD, iid, pUnkOuter).to_raw()
        }
        pub unsafe fn IDirectDraw2_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw2::CreateSurface(
                machine,
                this,
                desc,
                lplpDDSurface,
                pUnkOuter,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw2_EnumDisplayModes(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::ddraw::IDirectDraw2::EnumDisplayModes(
                        machine,
                        this,
                        dwFlags,
                        lpSurfaceDesc,
                        lpContext,
                        lpEnumCallback,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 20u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::ddraw::IDirectDraw2::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn IDirectDraw2_GetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDraw2::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw2_QueryInterface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let riid = <Option<&GUID>>::from_stack(mem, esp + 8u32);
            let ppvObject = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw2::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw2_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw2_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw2::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
        pub unsafe fn IDirectDraw7_CreatePalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let flags = <Result<DDPCAPS, u32>>::from_stack(mem, esp + 8u32);
            let entries = <u32>::from_stack(mem, esp + 12u32);
            let lplpPalette = <u32>::from_stack(mem, esp + 16u32);
            let unused = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDraw7::CreatePalette(
                machine,
                this,
                flags,
                entries,
                lplpPalette,
                unused,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let unused = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw7::CreateSurface(
                machine,
                this,
                desc,
                lpDirectDrawSurface7,
                unused,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_EnumDisplayModes(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC2>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::ddraw::IDirectDraw7::EnumDisplayModes(
                        machine,
                        this,
                        dwFlags,
                        lpSurfaceDesc,
                        lpContext,
                        lpEnumCallback,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 20u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::ddraw::IDirectDraw7::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn IDirectDraw7_GetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSurfaceDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDraw7::GetDisplayMode(machine, this, lpDDSurfaceDesc).to_raw()
        }
        pub unsafe fn IDirectDraw7_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_RestoreDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw7::RestoreDisplayMode(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetCooperativeLevel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let hwnd = <HWND>::from_stack(mem, esp + 8u32);
            let flags = <Result<DDSCL, u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw7::SetCooperativeLevel(machine, this, hwnd, flags).to_raw()
        }
        pub unsafe fn IDirectDraw7_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            let refresh = <u32>::from_stack(mem, esp + 20u32);
            let flags = <u32>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDraw7::SetDisplayMode(
                machine, this, width, height, bpp, refresh, flags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDraw7_WaitForVerticalBlank(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let flags = <u32>::from_stack(mem, esp + 8u32);
            let _unused = <u32>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw7::WaitForVerticalBlank(machine, this, flags, _unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawClipper_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawClipper::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawClipper_SetHWnd(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let unused = <u32>::from_stack(mem, esp + 8u32);
            let hwnd = <HWND>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawClipper::SetHWnd(machine, this, unused, hwnd).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawPalette::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawPalette_SetEntries(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let unused = <u32>::from_stack(mem, esp + 8u32);
            let start = <u32>::from_stack(mem, esp + 12u32);
            let count = <u32>::from_stack(mem, esp + 16u32);
            let entries = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawPalette::SetEntries(
                machine, this, unused, start, count, entries,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface2::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_GetSurfaceDesc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::GetSurfaceDesc(machine, this, desc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let event = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface2::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface2::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface2_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let ptr = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface2::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Blt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDstRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let lpSrc = <u32>::from_stack(mem, esp + 12u32);
            let lpSrcRect = <Option<&RECT>>::from_stack(mem, esp + 16u32);
            let flags = <Result<DDBLT, u32>>::from_stack(mem, esp + 20u32);
            let lpDDBLTFX = <Option<&DDBLTFX>>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDrawSurface7::Blt(
                machine, this, lpDstRect, lpSrc, lpSrcRect, flags, lpDDBLTFX,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_BltFast(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpSrc = <u32>::from_stack(mem, esp + 16u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 20u32);
            let flags = <u32>::from_stack(mem, esp + 24u32);
            winapi::ddraw::IDirectDrawSurface7::BltFast(machine, this, x, y, lpSrc, lpRect, flags)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Flip(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpSurf = <u32>::from_stack(mem, esp + 8u32);
            let flags = <Result<DDFLIP, u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface7::Flip(machine, this, lpSurf, flags).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps2 = <Option<&DDSCAPS2>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface7 = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface7::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps2,
                lpDirectDrawSurface7,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS2 = <Option<&mut DDSCAPS2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetCaps(machine, this, lpDDSCAPS2).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpHDC = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetDC(machine, this, lpHDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetPixelFormat(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&mut DDPIXELFORMAT>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetPixelFormat(machine, this, fmt).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_GetSurfaceDesc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDesc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::GetSurfaceDesc(machine, this, lpDesc).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC2>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let unused = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface7::Lock(machine, this, rect, desc, flags, unused)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface7::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, esp + 4u32);
            let _hDC = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::ReleaseDC(machine, _this, _hDC).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Restore(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface7::Restore(machine, _this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetClipper(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let clipper = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::SetClipper(machine, this, clipper).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_SetPalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let palette = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::SetPalette(machine, this, palette).to_raw()
        }
        pub unsafe fn IDirectDrawSurface7_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface7::Unlock(machine, this, rect).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetAttachedSurface(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCaps = <Option<&DDSCAPS>>::from_stack(mem, esp + 8u32);
            let lpDirectDrawSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDrawSurface::GetAttachedSurface(
                machine,
                this,
                lpDDSCaps,
                lpDirectDrawSurface,
            )
            .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_GetCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpDDSCAPS = <Option<&mut DDSCAPS>>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface::GetCaps(machine, this, lpDDSCAPS).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let rect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let desc = <Option<&mut DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let flags = <Result<DDLOCK, u32>>::from_stack(mem, esp + 16u32);
            let event = <u32>::from_stack(mem, esp + 20u32);
            winapi::ddraw::IDirectDrawSurface::Lock(machine, this, rect, desc, flags, event)
                .to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDrawSurface::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDrawSurface_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let ptr = <u32>::from_stack(mem, esp + 8u32);
            winapi::ddraw::IDirectDrawSurface::Unlock(machine, this, ptr).to_raw()
        }
        pub unsafe fn IDirectDraw_CreateSurface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let desc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 8u32);
            let lplpDDSurface = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw::CreateSurface(machine, this, desc, lplpDDSurface, pUnkOuter)
                .to_raw()
        }
        pub unsafe fn IDirectDraw_EnumDisplayModes(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpSurfaceDesc = <Option<&DDSURFACEDESC>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            let lpEnumCallback = <u32>::from_stack(mem, esp + 20u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::ddraw::IDirectDraw::EnumDisplayModes(
                        machine,
                        this,
                        dwFlags,
                        lpSurfaceDesc,
                        lpContext,
                        lpEnumCallback,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 20u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::ddraw::IDirectDraw::EnumDisplayModes(
                    machine,
                    this,
                    dwFlags,
                    lpSurfaceDesc,
                    lpContext,
                    lpEnumCallback
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn IDirectDraw_QueryInterface(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let riid = <Option<&GUID>>::from_stack(mem, esp + 8u32);
            let ppvObject = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::ddraw::IDirectDraw::QueryInterface(machine, this, riid, ppvObject).to_raw()
        }
        pub unsafe fn IDirectDraw_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::ddraw::IDirectDraw::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectDraw_SetDisplayMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let width = <u32>::from_stack(mem, esp + 8u32);
            let height = <u32>::from_stack(mem, esp + 12u32);
            let bpp = <u32>::from_stack(mem, esp + 16u32);
            winapi::ddraw::IDirectDraw::SetDisplayMode(machine, this, width, height, bpp).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const DirectDrawCreate: Shim = Shim {
            name: "DirectDrawCreate",
            func: impls::DirectDrawCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectDrawCreateClipper: Shim = Shim {
            name: "DirectDrawCreateClipper",
            func: impls::DirectDrawCreateClipper,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectDrawCreateEx: Shim = Shim {
            name: "DirectDrawCreateEx",
            func: impls::DirectDrawCreateEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectDraw2_CreateSurface: Shim = Shim {
            name: "IDirectDraw2::CreateSurface",
            func: impls::IDirectDraw2_CreateSurface,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectDraw2_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw2::EnumDisplayModes",
            func: impls::IDirectDraw2_EnumDisplayModes,
            stack_consumed: 20u32,
            is_async: true,
        };
        pub const IDirectDraw2_GetDisplayMode: Shim = Shim {
            name: "IDirectDraw2::GetDisplayMode",
            func: impls::IDirectDraw2_GetDisplayMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDraw2_QueryInterface: Shim = Shim {
            name: "IDirectDraw2::QueryInterface",
            func: impls::IDirectDraw2_QueryInterface,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDraw2_Release: Shim = Shim {
            name: "IDirectDraw2::Release",
            func: impls::IDirectDraw2_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDraw2_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw2::SetDisplayMode",
            func: impls::IDirectDraw2_SetDisplayMode,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectDraw7_CreatePalette: Shim = Shim {
            name: "IDirectDraw7::CreatePalette",
            func: impls::IDirectDraw7_CreatePalette,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectDraw7_CreateSurface: Shim = Shim {
            name: "IDirectDraw7::CreateSurface",
            func: impls::IDirectDraw7_CreateSurface,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectDraw7_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw7::EnumDisplayModes",
            func: impls::IDirectDraw7_EnumDisplayModes,
            stack_consumed: 20u32,
            is_async: true,
        };
        pub const IDirectDraw7_GetDisplayMode: Shim = Shim {
            name: "IDirectDraw7::GetDisplayMode",
            func: impls::IDirectDraw7_GetDisplayMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDraw7_Release: Shim = Shim {
            name: "IDirectDraw7::Release",
            func: impls::IDirectDraw7_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDraw7_RestoreDisplayMode: Shim = Shim {
            name: "IDirectDraw7::RestoreDisplayMode",
            func: impls::IDirectDraw7_RestoreDisplayMode,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDraw7_SetCooperativeLevel: Shim = Shim {
            name: "IDirectDraw7::SetCooperativeLevel",
            func: impls::IDirectDraw7_SetCooperativeLevel,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDraw7_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw7::SetDisplayMode",
            func: impls::IDirectDraw7_SetDisplayMode,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const IDirectDraw7_WaitForVerticalBlank: Shim = Shim {
            name: "IDirectDraw7::WaitForVerticalBlank",
            func: impls::IDirectDraw7_WaitForVerticalBlank,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawClipper_Release: Shim = Shim {
            name: "IDirectDrawClipper::Release",
            func: impls::IDirectDrawClipper_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawClipper_SetHWnd: Shim = Shim {
            name: "IDirectDrawClipper::SetHWnd",
            func: impls::IDirectDrawClipper_SetHWnd,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawPalette_Release: Shim = Shim {
            name: "IDirectDrawPalette::Release",
            func: impls::IDirectDrawPalette_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawPalette_SetEntries: Shim = Shim {
            name: "IDirectDrawPalette::SetEntries",
            func: impls::IDirectDrawPalette_SetEntries,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface2::GetAttachedSurface",
            func: impls::IDirectDrawSurface2_GetAttachedSurface,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface2::GetCaps",
            func: impls::IDirectDrawSurface2_GetCaps,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_GetSurfaceDesc: Shim = Shim {
            name: "IDirectDrawSurface2::GetSurfaceDesc",
            func: impls::IDirectDrawSurface2_GetSurfaceDesc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_Lock: Shim = Shim {
            name: "IDirectDrawSurface2::Lock",
            func: impls::IDirectDrawSurface2_Lock,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_Release: Shim = Shim {
            name: "IDirectDrawSurface2::Release",
            func: impls::IDirectDrawSurface2_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawSurface2_Unlock: Shim = Shim {
            name: "IDirectDrawSurface2::Unlock",
            func: impls::IDirectDrawSurface2_Unlock,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Blt: Shim = Shim {
            name: "IDirectDrawSurface7::Blt",
            func: impls::IDirectDrawSurface7_Blt,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_BltFast: Shim = Shim {
            name: "IDirectDrawSurface7::BltFast",
            func: impls::IDirectDrawSurface7_BltFast,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Flip: Shim = Shim {
            name: "IDirectDrawSurface7::Flip",
            func: impls::IDirectDrawSurface7_Flip,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface7::GetAttachedSurface",
            func: impls::IDirectDrawSurface7_GetAttachedSurface,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface7::GetCaps",
            func: impls::IDirectDrawSurface7_GetCaps,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_GetDC: Shim = Shim {
            name: "IDirectDrawSurface7::GetDC",
            func: impls::IDirectDrawSurface7_GetDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_GetPixelFormat: Shim = Shim {
            name: "IDirectDrawSurface7::GetPixelFormat",
            func: impls::IDirectDrawSurface7_GetPixelFormat,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_GetSurfaceDesc: Shim = Shim {
            name: "IDirectDrawSurface7::GetSurfaceDesc",
            func: impls::IDirectDrawSurface7_GetSurfaceDesc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Lock: Shim = Shim {
            name: "IDirectDrawSurface7::Lock",
            func: impls::IDirectDrawSurface7_Lock,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Release: Shim = Shim {
            name: "IDirectDrawSurface7::Release",
            func: impls::IDirectDrawSurface7_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_ReleaseDC: Shim = Shim {
            name: "IDirectDrawSurface7::ReleaseDC",
            func: impls::IDirectDrawSurface7_ReleaseDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Restore: Shim = Shim {
            name: "IDirectDrawSurface7::Restore",
            func: impls::IDirectDrawSurface7_Restore,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_SetClipper: Shim = Shim {
            name: "IDirectDrawSurface7::SetClipper",
            func: impls::IDirectDrawSurface7_SetClipper,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_SetPalette: Shim = Shim {
            name: "IDirectDrawSurface7::SetPalette",
            func: impls::IDirectDrawSurface7_SetPalette,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface7_Unlock: Shim = Shim {
            name: "IDirectDrawSurface7::Unlock",
            func: impls::IDirectDrawSurface7_Unlock,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface_GetAttachedSurface: Shim = Shim {
            name: "IDirectDrawSurface::GetAttachedSurface",
            func: impls::IDirectDrawSurface_GetAttachedSurface,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDrawSurface_GetCaps: Shim = Shim {
            name: "IDirectDrawSurface::GetCaps",
            func: impls::IDirectDrawSurface_GetCaps,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDrawSurface_Lock: Shim = Shim {
            name: "IDirectDrawSurface::Lock",
            func: impls::IDirectDrawSurface_Lock,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectDrawSurface_Release: Shim = Shim {
            name: "IDirectDrawSurface::Release",
            func: impls::IDirectDrawSurface_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDrawSurface_Unlock: Shim = Shim {
            name: "IDirectDrawSurface::Unlock",
            func: impls::IDirectDrawSurface_Unlock,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectDraw_CreateSurface: Shim = Shim {
            name: "IDirectDraw::CreateSurface",
            func: impls::IDirectDraw_CreateSurface,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectDraw_EnumDisplayModes: Shim = Shim {
            name: "IDirectDraw::EnumDisplayModes",
            func: impls::IDirectDraw_EnumDisplayModes,
            stack_consumed: 20u32,
            is_async: true,
        };
        pub const IDirectDraw_QueryInterface: Shim = Shim {
            name: "IDirectDraw::QueryInterface",
            func: impls::IDirectDraw_QueryInterface,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectDraw_Release: Shim = Shim {
            name: "IDirectDraw::Release",
            func: impls::IDirectDraw_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectDraw_SetDisplayMode: Shim = Shim {
            name: "IDirectDraw::SetDisplayMode",
            func: impls::IDirectDraw_SetDisplayMode,
            stack_consumed: 16u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 53usize] = [
        shims::DirectDrawCreate,
        shims::DirectDrawCreateClipper,
        shims::DirectDrawCreateEx,
        shims::IDirectDraw2_CreateSurface,
        shims::IDirectDraw2_EnumDisplayModes,
        shims::IDirectDraw2_GetDisplayMode,
        shims::IDirectDraw2_QueryInterface,
        shims::IDirectDraw2_Release,
        shims::IDirectDraw2_SetDisplayMode,
        shims::IDirectDraw7_CreatePalette,
        shims::IDirectDraw7_CreateSurface,
        shims::IDirectDraw7_EnumDisplayModes,
        shims::IDirectDraw7_GetDisplayMode,
        shims::IDirectDraw7_Release,
        shims::IDirectDraw7_RestoreDisplayMode,
        shims::IDirectDraw7_SetCooperativeLevel,
        shims::IDirectDraw7_SetDisplayMode,
        shims::IDirectDraw7_WaitForVerticalBlank,
        shims::IDirectDrawClipper_Release,
        shims::IDirectDrawClipper_SetHWnd,
        shims::IDirectDrawPalette_Release,
        shims::IDirectDrawPalette_SetEntries,
        shims::IDirectDrawSurface2_GetAttachedSurface,
        shims::IDirectDrawSurface2_GetCaps,
        shims::IDirectDrawSurface2_GetSurfaceDesc,
        shims::IDirectDrawSurface2_Lock,
        shims::IDirectDrawSurface2_Release,
        shims::IDirectDrawSurface2_Unlock,
        shims::IDirectDrawSurface7_Blt,
        shims::IDirectDrawSurface7_BltFast,
        shims::IDirectDrawSurface7_Flip,
        shims::IDirectDrawSurface7_GetAttachedSurface,
        shims::IDirectDrawSurface7_GetCaps,
        shims::IDirectDrawSurface7_GetDC,
        shims::IDirectDrawSurface7_GetPixelFormat,
        shims::IDirectDrawSurface7_GetSurfaceDesc,
        shims::IDirectDrawSurface7_Lock,
        shims::IDirectDrawSurface7_Release,
        shims::IDirectDrawSurface7_ReleaseDC,
        shims::IDirectDrawSurface7_Restore,
        shims::IDirectDrawSurface7_SetClipper,
        shims::IDirectDrawSurface7_SetPalette,
        shims::IDirectDrawSurface7_Unlock,
        shims::IDirectDrawSurface_GetAttachedSurface,
        shims::IDirectDrawSurface_GetCaps,
        shims::IDirectDrawSurface_Lock,
        shims::IDirectDrawSurface_Release,
        shims::IDirectDrawSurface_Unlock,
        shims::IDirectDraw_CreateSurface,
        shims::IDirectDraw_EnumDisplayModes,
        shims::IDirectDraw_QueryInterface,
        shims::IDirectDraw_Release,
        shims::IDirectDraw_SetDisplayMode,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ddraw.dll"),
    };
}
pub mod dsound {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::dsound::*;
        pub unsafe fn DirectSoundCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, esp + 4u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter).to_raw()
        }
        pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, esp + 4u32);
            let lpContext = <u32>::from_stack(mem, esp + 8u32);
            winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            winapi::dsound::IDirectSoundBuffer::GetCurrentPosition(
                machine,
                this,
                lpdwCurrentPlayCursor,
                lpdwCurrentWriteCursor,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_GetStatus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpdwStatus = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::dsound::IDirectSoundBuffer::GetStatus(machine, this, lpdwStatus).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwWriteCursor = <u32>::from_stack(mem, esp + 8u32);
            let dwWriteBytes = <u32>::from_stack(mem, esp + 12u32);
            let lplpvAudioPtr1 = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpdwAudioBytes1 = <Option<&mut u32>>::from_stack(mem, esp + 20u32);
            let lplpvAudioPtr2 = <Option<&mut u32>>::from_stack(mem, esp + 24u32);
            let lpdwAudioBytes2 = <Option<&mut u32>>::from_stack(mem, esp + 28u32);
            let dwFlags = <Result<DSBLOCK, u32>>::from_stack(mem, esp + 32u32);
            winapi::dsound::IDirectSoundBuffer::Lock(
                machine,
                this,
                dwWriteCursor,
                dwWriteBytes,
                lplpvAudioPtr1,
                lpdwAudioBytes1,
                lplpvAudioPtr2,
                lpdwAudioBytes2,
                dwFlags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Play(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let dwReserved1 = <u32>::from_stack(mem, esp + 8u32);
            let dwReserved2 = <u32>::from_stack(mem, esp + 12u32);
            let dwFlags = <u32>::from_stack(mem, esp + 16u32);
            winapi::dsound::IDirectSoundBuffer::Play(
                machine,
                this,
                dwReserved1,
                dwReserved2,
                dwFlags,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::dsound::IDirectSoundBuffer::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_SetFormat(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 8u32);
            winapi::dsound::IDirectSoundBuffer::SetFormat(machine, this, lpcfxFormat).to_raw()
        }
        pub unsafe fn IDirectSoundBuffer_Unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpvAudioPtr1 = <u32>::from_stack(mem, esp + 8u32);
            let dwAudioBytes1 = <u32>::from_stack(mem, esp + 12u32);
            let lpvAudioPtr2 = <u32>::from_stack(mem, esp + 16u32);
            let dwAudioBytes2 = <u32>::from_stack(mem, esp + 20u32);
            winapi::dsound::IDirectSoundBuffer::Unlock(
                machine,
                this,
                lpvAudioPtr1,
                dwAudioBytes1,
                lpvAudioPtr2,
                dwAudioBytes2,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSound_CreateSoundBuffer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let lpcDSBufferDesc = <Option<&DSBUFFERDESC>>::from_stack(mem, esp + 8u32);
            let lplpDirectSoundBuffer = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let pUnkOuter = <u32>::from_stack(mem, esp + 16u32);
            winapi::dsound::IDirectSound::CreateSoundBuffer(
                machine,
                this,
                lpcDSBufferDesc,
                lplpDirectSoundBuffer,
                pUnkOuter,
            )
            .to_raw()
        }
        pub unsafe fn IDirectSound_Release(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            winapi::dsound::IDirectSound::Release(machine, this).to_raw()
        }
        pub unsafe fn IDirectSound_SetCooperativeLevel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let this = <u32>::from_stack(mem, esp + 4u32);
            let hwnd = <u32>::from_stack(mem, esp + 8u32);
            let dwLevel = <u32>::from_stack(mem, esp + 12u32);
            winapi::dsound::IDirectSound::SetCooperativeLevel(machine, this, hwnd, dwLevel).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const DirectSoundCreate: Shim = Shim {
            name: "DirectSoundCreate",
            func: impls::DirectSoundCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const DirectSoundEnumerateA: Shim = Shim {
            name: "DirectSoundEnumerateA",
            func: impls::DirectSoundEnumerateA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_GetCurrentPosition: Shim = Shim {
            name: "IDirectSoundBuffer::GetCurrentPosition",
            func: impls::IDirectSoundBuffer_GetCurrentPosition,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_GetStatus: Shim = Shim {
            name: "IDirectSoundBuffer::GetStatus",
            func: impls::IDirectSoundBuffer_GetStatus,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_Lock: Shim = Shim {
            name: "IDirectSoundBuffer::Lock",
            func: impls::IDirectSoundBuffer_Lock,
            stack_consumed: 32u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_Play: Shim = Shim {
            name: "IDirectSoundBuffer::Play",
            func: impls::IDirectSoundBuffer_Play,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_Release: Shim = Shim {
            name: "IDirectSoundBuffer::Release",
            func: impls::IDirectSoundBuffer_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_SetFormat: Shim = Shim {
            name: "IDirectSoundBuffer::SetFormat",
            func: impls::IDirectSoundBuffer_SetFormat,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IDirectSoundBuffer_Unlock: Shim = Shim {
            name: "IDirectSoundBuffer::Unlock",
            func: impls::IDirectSoundBuffer_Unlock,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const IDirectSound_CreateSoundBuffer: Shim = Shim {
            name: "IDirectSound::CreateSoundBuffer",
            func: impls::IDirectSound_CreateSoundBuffer,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const IDirectSound_Release: Shim = Shim {
            name: "IDirectSound::Release",
            func: impls::IDirectSound_Release,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IDirectSound_SetCooperativeLevel: Shim = Shim {
            name: "IDirectSound::SetCooperativeLevel",
            func: impls::IDirectSound_SetCooperativeLevel,
            stack_consumed: 12u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 12usize] = [
        shims::DirectSoundCreate,
        shims::DirectSoundEnumerateA,
        shims::IDirectSoundBuffer_GetCurrentPosition,
        shims::IDirectSoundBuffer_GetStatus,
        shims::IDirectSoundBuffer_Lock,
        shims::IDirectSoundBuffer_Play,
        shims::IDirectSoundBuffer_Release,
        shims::IDirectSoundBuffer_SetFormat,
        shims::IDirectSoundBuffer_Unlock,
        shims::IDirectSound_CreateSoundBuffer,
        shims::IDirectSound_Release,
        shims::IDirectSound_SetCooperativeLevel,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/dsound.dll"),
    };
}
pub mod gdi32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::gdi32::*;
        pub unsafe fn BitBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let x1 = <i32>::from_stack(mem, esp + 28u32);
            let y1 = <i32>::from_stack(mem, esp + 32u32);
            let rop = <u32>::from_stack(mem, esp + 36u32);
            winapi::gdi32::BitBlt(machine, hdc, x, y, cx, cy, hdcSrc, x1, y1, rop).to_raw()
        }
        pub unsafe fn CreateBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nWidth = <u32>::from_stack(mem, esp + 4u32);
            let nHeight = <u32>::from_stack(mem, esp + 8u32);
            let nPlanes = <u32>::from_stack(mem, esp + 12u32);
            let nBitCount = <u32>::from_stack(mem, esp + 16u32);
            let lpBits = <u32>::from_stack(mem, esp + 20u32);
            winapi::gdi32::CreateBitmap(machine, nWidth, nHeight, nPlanes, nBitCount, lpBits)
                .to_raw()
        }
        pub unsafe fn CreateCompatibleBitmap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let cx = <u32>::from_stack(mem, esp + 8u32);
            let cy = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreateCompatibleBitmap(machine, hdc, cx, cy).to_raw()
        }
        pub unsafe fn CreateCompatibleDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateCompatibleDC(machine, hdc).to_raw()
        }
        pub unsafe fn CreateDIBSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let pbmi = <Option<&BITMAPINFOHEADER>>::from_stack(mem, esp + 8u32);
            let usage = <u32>::from_stack(mem, esp + 12u32);
            let ppvBits = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let hSection = <u32>::from_stack(mem, esp + 20u32);
            let offset = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::CreateDIBSection(machine, hdc, pbmi, usage, ppvBits, hSection, offset)
                .to_raw()
        }
        pub unsafe fn CreateFontA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let cHeight = <i32>::from_stack(mem, esp + 4u32);
            let cWidth = <i32>::from_stack(mem, esp + 8u32);
            let cEscapement = <i32>::from_stack(mem, esp + 12u32);
            let cOrientation = <i32>::from_stack(mem, esp + 16u32);
            let cWeight = <u32>::from_stack(mem, esp + 20u32);
            let bItalic = <u32>::from_stack(mem, esp + 24u32);
            let bUnderline = <u32>::from_stack(mem, esp + 28u32);
            let bStrikeOut = <u32>::from_stack(mem, esp + 32u32);
            let iCharSet = <u32>::from_stack(mem, esp + 36u32);
            let iOutPrecision = <u32>::from_stack(mem, esp + 40u32);
            let iClipPrecision = <u32>::from_stack(mem, esp + 44u32);
            let iQuality = <u32>::from_stack(mem, esp + 48u32);
            let iPitchAndFamily = <u32>::from_stack(mem, esp + 52u32);
            let pszFaceName = <Option<&str>>::from_stack(mem, esp + 56u32);
            winapi::gdi32::CreateFontA(
                machine,
                cHeight,
                cWidth,
                cEscapement,
                cOrientation,
                cWeight,
                bItalic,
                bUnderline,
                bStrikeOut,
                iCharSet,
                iOutPrecision,
                iClipPrecision,
                iQuality,
                iPitchAndFamily,
                pszFaceName,
            )
            .to_raw()
        }
        pub unsafe fn CreatePalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let plpal = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreatePalette(machine, plpal).to_raw()
        }
        pub unsafe fn CreatePen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let iStyle = <Result<PS, u32>>::from_stack(mem, esp + 4u32);
            let cWidth = <u32>::from_stack(mem, esp + 8u32);
            let color = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::CreatePen(machine, iStyle, cWidth, color).to_raw()
        }
        pub unsafe fn CreateSolidBrush(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let color = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::CreateSolidBrush(machine, color).to_raw()
        }
        pub unsafe fn DeleteDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <u32>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteDC(machine, hdc).to_raw()
        }
        pub unsafe fn DeleteObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            winapi::gdi32::DeleteObject(machine, handle).to_raw()
        }
        pub unsafe fn GetDCOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDCOrgEx(machine, hdc, lpPoint).to_raw()
        }
        pub unsafe fn GetDeviceCaps(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let index = <Result<GetDeviceCapsArg, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetDeviceCaps(machine, hdc, index).to_raw()
        }
        pub unsafe fn GetLayout(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetLayout(machine, hdc).to_raw()
        }
        pub unsafe fn GetObjectA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let handle = <HGDIOBJ>::from_stack(mem, esp + 4u32);
            let bytes = <u32>::from_stack(mem, esp + 8u32);
            let out = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetObjectA(machine, handle, bytes, out).to_raw()
        }
        pub unsafe fn GetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::GetPixel(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn GetStockObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let i = <Result<GetStockObjectArg, u32>>::from_stack(mem, esp + 4u32);
            winapi::gdi32::GetStockObject(machine, i).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32A(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32A(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextExtentPoint32W(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            let c = <i32>::from_stack(mem, esp + 12u32);
            let psizl = <Option<&mut SIZE>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::GetTextExtentPoint32W(machine, hdc, lpString, c, psizl).to_raw()
        }
        pub unsafe fn GetTextMetricsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICA>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsA(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn GetTextMetricsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let lptm = <Option<&mut TEXTMETRICW>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::GetTextMetricsW(machine, hdc, lptm).to_raw()
        }
        pub unsafe fn LineDDA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let xStart = <i32>::from_stack(mem, esp + 4u32);
            let yStart = <i32>::from_stack(mem, esp + 8u32);
            let xEnd = <i32>::from_stack(mem, esp + 12u32);
            let yEnd = <i32>::from_stack(mem, esp + 16u32);
            let lpProc = <u32>::from_stack(mem, esp + 20u32);
            let data = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::LineDDA(machine, xStart, yStart, xEnd, yEnd, lpProc, data).to_raw()
        }
        pub unsafe fn LineTo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::LineTo(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn MoveToEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::MoveToEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn PatBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let w = <i32>::from_stack(mem, esp + 16u32);
            let h = <i32>::from_stack(mem, esp + 20u32);
            let rop = <u32>::from_stack(mem, esp + 24u32);
            winapi::gdi32::PatBlt(machine, hdc, x, y, w, h, rop).to_raw()
        }
        pub unsafe fn PtVisible(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            winapi::gdi32::PtVisible(machine, hdc, x, y).to_raw()
        }
        pub unsafe fn RealizePalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            winapi::gdi32::RealizePalette(machine, hdc).to_raw()
        }
        pub unsafe fn SelectObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let hGdiObj = <HGDIOBJ>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SelectObject(machine, hdc, hGdiObj).to_raw()
        }
        pub unsafe fn SelectPalette(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let hpal = <HGDIOBJ>::from_stack(mem, esp + 8u32);
            let bForceBackground = <bool>::from_stack(mem, esp + 12u32);
            winapi::gdi32::SelectPalette(machine, hdc, hpal, bForceBackground).to_raw()
        }
        pub unsafe fn SetBkColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn SetBkMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let mode = <i32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetBkMode(machine, hdc, mode).to_raw()
        }
        pub unsafe fn SetBrushOrgEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <i32>::from_stack(mem, esp + 8u32);
            let y = <i32>::from_stack(mem, esp + 12u32);
            let lppt = <Option<&mut POINT>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetBrushOrgEx(machine, hdc, x, y, lppt).to_raw()
        }
        pub unsafe fn SetDIBitsToDevice(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let w = <u32>::from_stack(mem, esp + 16u32);
            let h = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let StartScan = <u32>::from_stack(mem, esp + 32u32);
            let cLines = <u32>::from_stack(mem, esp + 36u32);
            let lpvBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let ColorUse = <u32>::from_stack(mem, esp + 48u32);
            winapi::gdi32::SetDIBitsToDevice(
                machine, hdc, xDest, yDest, w, h, xSrc, ySrc, StartScan, cLines, lpvBits, lpbmi,
                ColorUse,
            )
            .to_raw()
        }
        pub unsafe fn SetPixel(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let color = <u32>::from_stack(mem, esp + 16u32);
            winapi::gdi32::SetPixel(machine, hdc, x, y, color).to_raw()
        }
        pub unsafe fn SetROP2(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let rop2 = <Result<R2, u32>>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetROP2(machine, hdc, rop2).to_raw()
        }
        pub unsafe fn SetTextColor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let color = <u32>::from_stack(mem, esp + 8u32);
            winapi::gdi32::SetTextColor(machine, hdc, color).to_raw()
        }
        pub unsafe fn StretchBlt(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdcDest = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <i32>::from_stack(mem, esp + 8u32);
            let yDest = <i32>::from_stack(mem, esp + 12u32);
            let wDest = <u32>::from_stack(mem, esp + 16u32);
            let hDest = <u32>::from_stack(mem, esp + 20u32);
            let hdcSrc = <HDC>::from_stack(mem, esp + 24u32);
            let xSrc = <i32>::from_stack(mem, esp + 28u32);
            let ySrc = <i32>::from_stack(mem, esp + 32u32);
            let wSrc = <u32>::from_stack(mem, esp + 36u32);
            let hSrc = <u32>::from_stack(mem, esp + 40u32);
            let rop = <u32>::from_stack(mem, esp + 44u32);
            winapi::gdi32::StretchBlt(
                machine, hdcDest, xDest, yDest, wDest, hDest, hdcSrc, xSrc, ySrc, wSrc, hSrc, rop,
            )
            .to_raw()
        }
        pub unsafe fn StretchDIBits(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let xDest = <u32>::from_stack(mem, esp + 8u32);
            let yDest = <u32>::from_stack(mem, esp + 12u32);
            let DestWidth = <u32>::from_stack(mem, esp + 16u32);
            let DestHeight = <u32>::from_stack(mem, esp + 20u32);
            let xSrc = <u32>::from_stack(mem, esp + 24u32);
            let ySrc = <u32>::from_stack(mem, esp + 28u32);
            let SrcWidth = <u32>::from_stack(mem, esp + 32u32);
            let SrcHeight = <u32>::from_stack(mem, esp + 36u32);
            let lpBits = <u32>::from_stack(mem, esp + 40u32);
            let lpbmi = <u32>::from_stack(mem, esp + 44u32);
            let iUsage = <u32>::from_stack(mem, esp + 48u32);
            let rop = <Result<RasterOp, u32>>::from_stack(mem, esp + 52u32);
            winapi::gdi32::StretchDIBits(
                machine, hdc, xDest, yDest, DestWidth, DestHeight, xSrc, ySrc, SrcWidth, SrcHeight,
                lpBits, lpbmi, iUsage, rop,
            )
            .to_raw()
        }
        pub unsafe fn TextOutA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u8>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutA(machine, hdc, x, y, lpString).to_raw()
        }
        pub unsafe fn TextOutW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hdc = <HDC>::from_stack(mem, esp + 4u32);
            let x = <u32>::from_stack(mem, esp + 8u32);
            let y = <u32>::from_stack(mem, esp + 12u32);
            let lpString = <ArrayWithSize<u16>>::from_stack(mem, esp + 16u32);
            winapi::gdi32::TextOutW(machine, hdc, x, y, lpString).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const BitBlt: Shim = Shim {
            name: "BitBlt",
            func: impls::BitBlt,
            stack_consumed: 36u32,
            is_async: false,
        };
        pub const CreateBitmap: Shim = Shim {
            name: "CreateBitmap",
            func: impls::CreateBitmap,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const CreateCompatibleBitmap: Shim = Shim {
            name: "CreateCompatibleBitmap",
            func: impls::CreateCompatibleBitmap,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const CreateCompatibleDC: Shim = Shim {
            name: "CreateCompatibleDC",
            func: impls::CreateCompatibleDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const CreateDIBSection: Shim = Shim {
            name: "CreateDIBSection",
            func: impls::CreateDIBSection,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const CreateFontA: Shim = Shim {
            name: "CreateFontA",
            func: impls::CreateFontA,
            stack_consumed: 56u32,
            is_async: false,
        };
        pub const CreatePalette: Shim = Shim {
            name: "CreatePalette",
            func: impls::CreatePalette,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const CreatePen: Shim = Shim {
            name: "CreatePen",
            func: impls::CreatePen,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const CreateSolidBrush: Shim = Shim {
            name: "CreateSolidBrush",
            func: impls::CreateSolidBrush,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DeleteDC: Shim = Shim {
            name: "DeleteDC",
            func: impls::DeleteDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DeleteObject: Shim = Shim {
            name: "DeleteObject",
            func: impls::DeleteObject,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetDCOrgEx: Shim = Shim {
            name: "GetDCOrgEx",
            func: impls::GetDCOrgEx,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetDeviceCaps: Shim = Shim {
            name: "GetDeviceCaps",
            func: impls::GetDeviceCaps,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetLayout: Shim = Shim {
            name: "GetLayout",
            func: impls::GetLayout,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetObjectA: Shim = Shim {
            name: "GetObjectA",
            func: impls::GetObjectA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetPixel: Shim = Shim {
            name: "GetPixel",
            func: impls::GetPixel,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetStockObject: Shim = Shim {
            name: "GetStockObject",
            func: impls::GetStockObject,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetTextExtentPoint32A: Shim = Shim {
            name: "GetTextExtentPoint32A",
            func: impls::GetTextExtentPoint32A,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetTextExtentPoint32W: Shim = Shim {
            name: "GetTextExtentPoint32W",
            func: impls::GetTextExtentPoint32W,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetTextMetricsA: Shim = Shim {
            name: "GetTextMetricsA",
            func: impls::GetTextMetricsA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetTextMetricsW: Shim = Shim {
            name: "GetTextMetricsW",
            func: impls::GetTextMetricsW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LineDDA: Shim = Shim {
            name: "LineDDA",
            func: impls::LineDDA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const LineTo: Shim = Shim {
            name: "LineTo",
            func: impls::LineTo,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const MoveToEx: Shim = Shim {
            name: "MoveToEx",
            func: impls::MoveToEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const PatBlt: Shim = Shim {
            name: "PatBlt",
            func: impls::PatBlt,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const PtVisible: Shim = Shim {
            name: "PtVisible",
            func: impls::PtVisible,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const RealizePalette: Shim = Shim {
            name: "RealizePalette",
            func: impls::RealizePalette,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SelectObject: Shim = Shim {
            name: "SelectObject",
            func: impls::SelectObject,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SelectPalette: Shim = Shim {
            name: "SelectPalette",
            func: impls::SelectPalette,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const SetBkColor: Shim = Shim {
            name: "SetBkColor",
            func: impls::SetBkColor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetBkMode: Shim = Shim {
            name: "SetBkMode",
            func: impls::SetBkMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetBrushOrgEx: Shim = Shim {
            name: "SetBrushOrgEx",
            func: impls::SetBrushOrgEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetDIBitsToDevice: Shim = Shim {
            name: "SetDIBitsToDevice",
            func: impls::SetDIBitsToDevice,
            stack_consumed: 48u32,
            is_async: false,
        };
        pub const SetPixel: Shim = Shim {
            name: "SetPixel",
            func: impls::SetPixel,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetROP2: Shim = Shim {
            name: "SetROP2",
            func: impls::SetROP2,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetTextColor: Shim = Shim {
            name: "SetTextColor",
            func: impls::SetTextColor,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const StretchBlt: Shim = Shim {
            name: "StretchBlt",
            func: impls::StretchBlt,
            stack_consumed: 44u32,
            is_async: false,
        };
        pub const StretchDIBits: Shim = Shim {
            name: "StretchDIBits",
            func: impls::StretchDIBits,
            stack_consumed: 52u32,
            is_async: false,
        };
        pub const TextOutA: Shim = Shim {
            name: "TextOutA",
            func: impls::TextOutA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const TextOutW: Shim = Shim {
            name: "TextOutW",
            func: impls::TextOutW,
            stack_consumed: 20u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 40usize] = [
        shims::BitBlt,
        shims::CreateBitmap,
        shims::CreateCompatibleBitmap,
        shims::CreateCompatibleDC,
        shims::CreateDIBSection,
        shims::CreateFontA,
        shims::CreatePalette,
        shims::CreatePen,
        shims::CreateSolidBrush,
        shims::DeleteDC,
        shims::DeleteObject,
        shims::GetDCOrgEx,
        shims::GetDeviceCaps,
        shims::GetLayout,
        shims::GetObjectA,
        shims::GetPixel,
        shims::GetStockObject,
        shims::GetTextExtentPoint32A,
        shims::GetTextExtentPoint32W,
        shims::GetTextMetricsA,
        shims::GetTextMetricsW,
        shims::LineDDA,
        shims::LineTo,
        shims::MoveToEx,
        shims::PatBlt,
        shims::PtVisible,
        shims::RealizePalette,
        shims::SelectObject,
        shims::SelectPalette,
        shims::SetBkColor,
        shims::SetBkMode,
        shims::SetBrushOrgEx,
        shims::SetDIBitsToDevice,
        shims::SetPixel,
        shims::SetROP2,
        shims::SetTextColor,
        shims::StretchBlt,
        shims::StretchDIBits,
        shims::TextOutA,
        shims::TextOutW,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/gdi32.dll"),
    };
}
pub mod kernel32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::kernel32::*;
        pub unsafe fn AcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn AcquireSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::AcquireSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn AddVectoredExceptionHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let first = <u32>::from_stack(mem, esp + 4u32);
            let handler = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::AddVectoredExceptionHandler(machine, first, handler).to_raw()
        }
        pub unsafe fn CloseHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hObject = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::CloseHandle(machine, hObject).to_raw()
        }
        pub unsafe fn CreateDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::CreateDirectoryA(machine, lpPathName, lpSecurityAttributes).to_raw()
        }
        pub unsafe fn CreateEventA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpEventAttributes = <u32>::from_stack(mem, esp + 4u32);
            let bManualReset = <bool>::from_stack(mem, esp + 8u32);
            let bInitialState = <bool>::from_stack(mem, esp + 12u32);
            let lpName = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::CreateEventA(
                machine,
                lpEventAttributes,
                bManualReset,
                bInitialState,
                lpName,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileA(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateFileW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let dwDesiredAccess = <u32>::from_stack(mem, esp + 8u32);
            let dwShareMode = <u32>::from_stack(mem, esp + 12u32);
            let lpSecurityAttributes = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationDisposition =
                <Result<CreationDisposition, u32>>::from_stack(mem, esp + 20u32);
            let dwFlagsAndAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 24u32);
            let hTemplateFile = <HFILE>::from_stack(mem, esp + 28u32);
            winapi::kernel32::CreateFileW(
                machine,
                lpFileName,
                dwDesiredAccess,
                dwShareMode,
                lpSecurityAttributes,
                dwCreationDisposition,
                dwFlagsAndAttributes,
                hTemplateFile,
            )
            .to_raw()
        }
        pub unsafe fn CreateThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpThreadAttributes = <u32>::from_stack(mem, esp + 4u32);
            let dwStackSize = <u32>::from_stack(mem, esp + 8u32);
            let lpStartAddress = <u32>::from_stack(mem, esp + 12u32);
            let lpParameter = <u32>::from_stack(mem, esp + 16u32);
            let dwCreationFlags = <u32>::from_stack(mem, esp + 20u32);
            let lpThreadId = <u32>::from_stack(mem, esp + 24u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::CreateThread(
                        machine,
                        lpThreadAttributes,
                        dwStackSize,
                        lpStartAddress,
                        lpParameter,
                        dwCreationFlags,
                        lpThreadId,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 24u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::CreateThread(
                    machine,
                    lpThreadAttributes,
                    dwStackSize,
                    lpStartAddress,
                    lpParameter,
                    dwCreationFlags,
                    lpThreadId
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DeleteCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn DeleteFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DeleteFileA(machine, lpFileName).to_raw()
        }
        pub unsafe fn DisableThreadLibraryCalls(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::DisableThreadLibraryCalls(machine, hLibModule).to_raw()
        }
        pub unsafe fn EnterCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::EnterCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn ExitProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uExitCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ExitProcess(machine, uExitCode).to_raw()
        }
        pub unsafe fn FileTimeToSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileTime = <Option<&FILETIME>>::from_stack(mem, esp + 4u32);
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FileTimeToSystemTime(machine, lpFileTime, lpSystemTime).to_raw()
        }
        pub unsafe fn FindClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FindClose(machine, hFindFile).to_raw()
        }
        pub unsafe fn FindFirstFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindFirstFileA(machine, lpFileName, lpFindFileData).to_raw()
        }
        pub unsafe fn FindNextFileA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFindFile = <HFIND>::from_stack(mem, esp + 4u32);
            let lpFindFileData = <Option<&mut WIN32_FIND_DATAA>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::FindNextFileA(machine, hFindFile, lpFindFileData).to_raw()
        }
        pub unsafe fn FindResourceA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&str>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceA(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FindResourceW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpName = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let lpType = <ResourceKey<&Str16>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::FindResourceW(machine, hModule, lpName, lpType).to_raw()
        }
        pub unsafe fn FormatMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
            winapi::kernel32::FormatMessageA(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FormatMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <Result<FormatMessageFlags, u32>>::from_stack(mem, esp + 4u32);
            let lpSource = <u32>::from_stack(mem, esp + 8u32);
            let dwMessageId = <u32>::from_stack(mem, esp + 12u32);
            let dwLanguageId = <u32>::from_stack(mem, esp + 16u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 20u32);
            let nSize = <u32>::from_stack(mem, esp + 24u32);
            let args = <u32>::from_stack(mem, esp + 28u32);
            winapi::kernel32::FormatMessageW(
                machine,
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                args,
            )
            .to_raw()
        }
        pub unsafe fn FreeEnvironmentStringsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _penv = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeEnvironmentStringsA(machine, _penv).to_raw()
        }
        pub unsafe fn FreeLibrary(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hLibModule = <HMODULE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::FreeLibrary(machine, hLibModule).to_raw()
        }
        pub unsafe fn GetACP(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetACP(machine).to_raw()
        }
        pub unsafe fn GetCPInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _CodePage = <u32>::from_stack(mem, esp + 4u32);
            let _lpCPInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCPInfo(machine, _CodePage, _lpCPInfo).to_raw()
        }
        pub unsafe fn GetCommandLineA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineA(machine).to_raw()
        }
        pub unsafe fn GetCommandLineW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCommandLineW(machine).to_raw()
        }
        pub unsafe fn GetConsoleMode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let lpMode = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleMode(machine, hConsoleHandle, lpMode).to_raw()
        }
        pub unsafe fn GetConsoleScreenBufferInfo(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpConsoleScreenBufferInfo =
                <Option<&mut CONSOLE_SCREEN_BUFFER_INFO>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetConsoleScreenBufferInfo(
                machine,
                _hConsoleOutput,
                lpConsoleScreenBufferInfo,
            )
            .to_raw()
        }
        pub unsafe fn GetCurrentDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nBufferLength = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetCurrentDirectoryA(machine, nBufferLength, lpBuffer).to_raw()
        }
        pub unsafe fn GetCurrentProcessId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentProcessId(machine).to_raw()
        }
        pub unsafe fn GetCurrentThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThread(machine).to_raw()
        }
        pub unsafe fn GetCurrentThreadId(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetCurrentThreadId(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStrings(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStrings(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentStringsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetEnvironmentStringsW(machine).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableA(machine, name, buf).to_raw()
        }
        pub unsafe fn GetEnvironmentVariableW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let buf = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetEnvironmentVariableW(machine, name, buf).to_raw()
        }
        pub unsafe fn GetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileAttributesA(machine, lpFileName).to_raw()
        }
        pub unsafe fn GetFileInformationByHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileInformation =
                <Option<&mut BY_HANDLE_FILE_INFORMATION>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileInformationByHandle(machine, hFile, lpFileInformation).to_raw()
        }
        pub unsafe fn GetFileSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpFileSizeHigh = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetFileSize(machine, hFile, lpFileSizeHigh).to_raw()
        }
        pub unsafe fn GetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn GetFileType(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetFileType(machine, hFile).to_raw()
        }
        pub unsafe fn GetFullPathNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameA(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetFullPathNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let nBufferLength = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let lpFilePart = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetFullPathNameW(
                machine,
                lpFileName,
                nBufferLength,
                lpBuffer,
                lpFilePart,
            )
            .to_raw()
        }
        pub unsafe fn GetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetLastError(machine).to_raw()
        }
        pub unsafe fn GetLocalTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetLocalTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetModuleFileNameA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let filename = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetModuleFileNameA(machine, hModule, filename).to_raw()
        }
        pub unsafe fn GetModuleFileNameW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let _lpFilename = <u32>::from_stack(mem, esp + 8u32);
            let _nSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleFileNameW(machine, hModule, _lpFilename, _nSize).to_raw()
        }
        pub unsafe fn GetModuleHandleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleA(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetModuleHandleExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwFlags = <u32>::from_stack(mem, esp + 4u32);
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let hModule = <Option<&mut HMODULE>>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetModuleHandleExW(machine, dwFlags, lpModuleName, hModule).to_raw()
        }
        pub unsafe fn GetModuleHandleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpModuleName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetModuleHandleW(machine, lpModuleName).to_raw()
        }
        pub unsafe fn GetPrivateProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <u32>::from_stack(mem, esp + 12u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetPrivateProfileIntW(
                machine, lpAppName, lpKeyName, nDefault, lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetPrivateProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            let lpFileName = <Option<&Str16>>::from_stack(mem, esp + 24u32);
            winapi::kernel32::GetPrivateProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
                lpFileName,
            )
            .to_raw()
        }
        pub unsafe fn GetProcAddress(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let lpProcName = <GetProcAddressArg>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetProcAddress(machine, hModule, lpProcName).to_raw()
        }
        pub unsafe fn GetProcessHeap(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetProcessHeap(machine).to_raw()
        }
        pub unsafe fn GetProfileIntW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nDefault = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GetProfileIntW(machine, lpAppName, lpKeyName, nDefault).to_raw()
        }
        pub unsafe fn GetProfileStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAppName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let lpKeyName = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpDefault = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let lpReturnedString = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::GetProfileStringW(
                machine,
                lpAppName,
                lpKeyName,
                lpDefault,
                lpReturnedString,
            )
            .to_raw()
        }
        pub unsafe fn GetStartupInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoA(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStartupInfoW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpStartupInfo = <Option<&mut STARTUPINFOA>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStartupInfoW(machine, lpStartupInfo).to_raw()
        }
        pub unsafe fn GetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetStdHandle(machine, nStdHandle).to_raw()
        }
        pub unsafe fn GetSystemDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetSystemDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GetSystemTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&mut SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTime(machine, lpSystemTime).to_raw()
        }
        pub unsafe fn GetSystemTimeAsFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTimeAsFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetSystemTimeAsFileTime(machine, lpSystemTimeAsFileTime).to_raw()
        }
        pub unsafe fn GetTickCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetTickCount(machine).to_raw()
        }
        pub unsafe fn GetTimeZoneInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpTimeZoneInformation =
                <Option<&mut TIME_ZONE_INFORMATION>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetTimeZoneInformation(machine, lpTimeZoneInformation).to_raw()
        }
        pub unsafe fn GetVersion(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::GetVersion(machine).to_raw()
        }
        pub unsafe fn GetVersionExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpVersionInformation = <Option<&mut OSVERSIONINFO>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GetVersionExA(machine, lpVersionInformation).to_raw()
        }
        pub unsafe fn GetWindowsDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpBuffer = <u32>::from_stack(mem, esp + 4u32);
            let uSize = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GetWindowsDirectoryA(machine, lpBuffer, uSize).to_raw()
        }
        pub unsafe fn GlobalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::GlobalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn GlobalFlags(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFlags(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::GlobalFree(machine, hMem).to_raw()
        }
        pub unsafe fn GlobalReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            let uFlags = <GMEM>::from_stack(mem, esp + 12u32);
            winapi::kernel32::GlobalReAlloc(machine, hMem, dwBytes, uFlags).to_raw()
        }
        pub unsafe fn HeapAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <Result<HeapAllocFlags, u32>>::from_stack(mem, esp + 8u32);
            let dwBytes = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapAlloc(machine, hHeap, dwFlags, dwBytes).to_raw()
        }
        pub unsafe fn HeapCreate(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let flOptions = <Result<HeapCreateFlags, u32>>::from_stack(mem, esp + 4u32);
            let dwInitialSize = <u32>::from_stack(mem, esp + 8u32);
            let dwMaximumSize = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapCreate(machine, flOptions, dwInitialSize, dwMaximumSize).to_raw()
        }
        pub unsafe fn HeapDestroy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::HeapDestroy(machine, hHeap).to_raw()
        }
        pub unsafe fn HeapFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapFree(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn HeapReAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            let dwBytes = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapReAlloc(machine, hHeap, dwFlags, lpMem, dwBytes).to_raw()
        }
        pub unsafe fn HeapSetInformation(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let HeapHandle = <u32>::from_stack(mem, esp + 4u32);
            let HeapInformationClass = <u32>::from_stack(mem, esp + 8u32);
            let HeapInformation = <u32>::from_stack(mem, esp + 12u32);
            let HeapInformationLength = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::HeapSetInformation(
                machine,
                HeapHandle,
                HeapInformationClass,
                HeapInformation,
                HeapInformationLength,
            )
            .to_raw()
        }
        pub unsafe fn HeapSize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHeap = <u32>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMem = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::HeapSize(machine, hHeap, dwFlags, lpMem).to_raw()
        }
        pub unsafe fn InitOnceBeginInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let fPending = <Option<&mut u32>>::from_stack(mem, esp + 12u32);
            let lpContext = <u32>::from_stack(mem, esp + 16u32);
            winapi::kernel32::InitOnceBeginInitialize(
                machine, lpInitOnce, dwFlags, fPending, lpContext,
            )
            .to_raw()
        }
        pub unsafe fn InitOnceComplete(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpInitOnce = <Option<&mut INIT_ONCE>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpContext = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitOnceComplete(machine, lpInitOnce, dwFlags, lpContext).to_raw()
        }
        pub unsafe fn InitializeCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn InitializeCriticalSectionAndSpinCount(
            machine: &mut Machine,
            esp: u32,
        ) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::InitializeCriticalSectionAndSpinCount(
                machine,
                lpCriticalSection,
                dwSpinCount,
            )
            .to_raw()
        }
        pub unsafe fn InitializeCriticalSectionEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            let dwSpinCount = <u32>::from_stack(mem, esp + 8u32);
            let flags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::InitializeCriticalSectionEx(
                machine,
                lpCriticalSection,
                dwSpinCount,
                flags,
            )
            .to_raw()
        }
        pub unsafe fn InitializeSListHead(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ListHead = <Option<&mut SLIST_HEADER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InitializeSListHead(machine, ListHead).to_raw()
        }
        pub unsafe fn InterlockedIncrement(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let addend = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::InterlockedIncrement(machine, addend).to_raw()
        }
        pub unsafe fn IsBadReadPtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadReadPtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsBadWritePtr(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lp = <u32>::from_stack(mem, esp + 4u32);
            let ucb = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsBadWritePtr(machine, lp, ucb).to_raw()
        }
        pub unsafe fn IsDBCSLeadByte(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsDBCSLeadByte(machine, _TestChar).to_raw()
        }
        pub unsafe fn IsDBCSLeadByteEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _TestChar = <u8>::from_stack(mem, esp + 4u32);
            let _CodePage = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::IsDBCSLeadByteEx(machine, _TestChar, _CodePage).to_raw()
        }
        pub unsafe fn IsDebuggerPresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::IsDebuggerPresent(machine).to_raw()
        }
        pub unsafe fn IsProcessorFeaturePresent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let feature = <Result<ProcessorFeature, u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsProcessorFeaturePresent(machine, feature).to_raw()
        }
        pub unsafe fn IsValidCodePage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::IsValidCodePage(machine, CodePage).to_raw()
        }
        pub unsafe fn LeaveCriticalSection(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpCriticalSection = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LeaveCriticalSection(machine, lpCriticalSection).to_raw()
        }
        pub unsafe fn LoadLibraryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let filename = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LoadLibraryA(machine, filename).to_raw()
        }
        pub unsafe fn LoadLibraryExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpLibFileName = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            let hFile = <HFILE>::from_stack(mem, esp + 8u32);
            let dwFlags = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::LoadLibraryExW(machine, lpLibFileName, hFile, dwFlags).to_raw()
        }
        pub unsafe fn LoadResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let hResInfo = <HRSRC>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LoadResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn LocalAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uFlags = <GMEM>::from_stack(mem, esp + 4u32);
            let dwBytes = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::LocalAlloc(machine, uFlags, dwBytes).to_raw()
        }
        pub unsafe fn LocalFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMem = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LocalFree(machine, hMem).to_raw()
        }
        pub unsafe fn LockResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hResData = <HRSRC>::from_stack(mem, esp + 4u32);
            winapi::kernel32::LockResource(machine, hResData).to_raw()
        }
        pub unsafe fn MulDiv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nNumber = <i32>::from_stack(mem, esp + 4u32);
            let nNumerator = <i32>::from_stack(mem, esp + 8u32);
            let nDenominator = <i32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::MulDiv(machine, nNumber, nNumerator, nDenominator).to_raw()
        }
        pub unsafe fn MultiByteToWideChar(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let CodePage = <Result<CP, u32>>::from_stack(mem, esp + 4u32);
            let dwFlags = <u32>::from_stack(mem, esp + 8u32);
            let lpMultiByteStr = <u32>::from_stack(mem, esp + 12u32);
            let cbMultiByte = <i32>::from_stack(mem, esp + 16u32);
            let lpWideCharStr = <ArrayWithSizeMut<u16>>::from_stack(mem, esp + 20u32);
            winapi::kernel32::MultiByteToWideChar(
                machine,
                CodePage,
                dwFlags,
                lpMultiByteStr,
                cbMultiByte,
                lpWideCharStr,
            )
            .to_raw()
        }
        pub unsafe fn NtCurrentTeb(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::NtCurrentTeb(machine).to_raw()
        }
        pub unsafe fn OutputDebugStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let msg = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::OutputDebugStringA(machine, msg).to_raw()
        }
        pub unsafe fn QueryPerformanceCounter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPerformanceCount = <Option<&mut LARGE_INTEGER>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceCounter(machine, lpPerformanceCount).to_raw()
        }
        pub unsafe fn QueryPerformanceFrequency(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFrequency = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::QueryPerformanceFrequency(machine, lpFrequency).to_raw()
        }
        pub unsafe fn ReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesRead = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::ReadFile(machine, hFile, lpBuffer, lpNumberOfBytesRead, lpOverlapped)
                .to_raw()
        }
        pub unsafe fn ReleaseSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn ReleaseSRWLockShared(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ReleaseSRWLockShared(machine, SRWLock).to_raw()
        }
        pub unsafe fn RemoveDirectoryA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpPathName = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::RemoveDirectoryA(machine, lpPathName).to_raw()
        }
        pub unsafe fn ResumeThread(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            winapi::kernel32::ResumeThread(machine, hThread).to_raw()
        }
        pub unsafe fn SetConsoleCtrlHandler(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _handlerRoutine = <DWORD>::from_stack(mem, esp + 4u32);
            let _add = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetConsoleCtrlHandler(machine, _handlerRoutine, _add).to_raw()
        }
        pub unsafe fn SetEndOfFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEndOfFile(machine, hFile).to_raw()
        }
        pub unsafe fn SetEnvironmentVariableA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let name = <Option<&str>>::from_stack(mem, esp + 4u32);
            let value = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetEnvironmentVariableA(machine, name, value).to_raw()
        }
        pub unsafe fn SetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hEvent = <HEVENT>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetEvent(machine, hEvent).to_raw()
        }
        pub unsafe fn SetFileAttributesA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpFileName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwFileAttributes = <Result<FileAttribute, u32>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetFileAttributesA(machine, lpFileName, dwFileAttributes).to_raw()
        }
        pub unsafe fn SetFilePointer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lDistanceToMove = <i32>::from_stack(mem, esp + 8u32);
            let lpDistanceToMoveHigh = <Option<&mut i32>>::from_stack(mem, esp + 12u32);
            let dwMoveMethod = <Result<FILE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFilePointer(
                machine,
                hFile,
                lDistanceToMove,
                lpDistanceToMoveHigh,
                dwMoveMethod,
            )
            .to_raw()
        }
        pub unsafe fn SetFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpCreationTime = <Option<&FILETIME>>::from_stack(mem, esp + 8u32);
            let lpLastAccessTime = <Option<&FILETIME>>::from_stack(mem, esp + 12u32);
            let lpLastWriteTime = <Option<&FILETIME>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::SetFileTime(
                machine,
                hFile,
                lpCreationTime,
                lpLastAccessTime,
                lpLastWriteTime,
            )
            .to_raw()
        }
        pub unsafe fn SetHandleCount(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uNumber = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetHandleCount(machine, uNumber).to_raw()
        }
        pub unsafe fn SetLastError(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwErrCode = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetLastError(machine, dwErrCode).to_raw()
        }
        pub unsafe fn SetPriorityClass(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hProcess = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let dwPriorityClass = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetPriorityClass(machine, hProcess, dwPriorityClass).to_raw()
        }
        pub unsafe fn SetStdHandle(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nStdHandle = <Result<STD, u32>>::from_stack(mem, esp + 4u32);
            let hHandle = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetStdHandle(machine, nStdHandle, hHandle).to_raw()
        }
        pub unsafe fn SetThreadDescription(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let lpThreadDescription = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadDescription(machine, hThread, lpThreadDescription).to_raw()
        }
        pub unsafe fn SetThreadPriority(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hThread = <HTHREAD>::from_stack(mem, esp + 4u32);
            let nPriority = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SetThreadPriority(machine, hThread, nPriority).to_raw()
        }
        pub unsafe fn SetThreadStackGuarantee(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let StackSizeInBytes = <Option<&mut u32>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetThreadStackGuarantee(machine, StackSizeInBytes).to_raw()
        }
        pub unsafe fn SetUnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _lpTopLevelExceptionFilter = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::SetUnhandledExceptionFilter(machine, _lpTopLevelExceptionFilter)
                .to_raw()
        }
        pub unsafe fn SizeofResource(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hModule = <HMODULE>::from_stack(mem, esp + 4u32);
            let hResInfo = <HRSRC>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SizeofResource(machine, hModule, hResInfo).to_raw()
        }
        pub unsafe fn Sleep(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwMilliseconds = <u32>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::Sleep(machine, dwMilliseconds).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 4u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::Sleep(machine, dwMilliseconds));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn SystemTimeToFileTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpSystemTime = <Option<&SYSTEMTIME>>::from_stack(mem, esp + 4u32);
            let lpFileTime = <Option<&mut FILETIME>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::SystemTimeToFileTime(machine, lpSystemTime, lpFileTime).to_raw()
        }
        pub unsafe fn TlsAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::kernel32::TlsAlloc(machine).to_raw()
        }
        pub unsafe fn TlsFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsFree(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsGetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TlsGetValue(machine, dwTlsIndex).to_raw()
        }
        pub unsafe fn TlsSetValue(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwTlsIndex = <u32>::from_stack(mem, esp + 4u32);
            let lpTlsValue = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::TlsSetValue(machine, dwTlsIndex, lpTlsValue).to_raw()
        }
        pub unsafe fn TryAcquireSRWLockExclusive(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let SRWLock = <Option<&mut SRWLOCK>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::TryAcquireSRWLockExclusive(machine, SRWLock).to_raw()
        }
        pub unsafe fn UnhandledExceptionFilter(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _exceptionInfo = <u32>::from_stack(mem, esp + 4u32);
            winapi::kernel32::UnhandledExceptionFilter(machine, _exceptionInfo).to_raw()
        }
        pub unsafe fn VirtualAlloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flAllocationType = <Result<MEM, u32>>::from_stack(mem, esp + 12u32);
            let flProtec = <Result<PAGE, u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualAlloc(machine, lpAddress, dwSize, flAllocationType, flProtec)
                .to_raw()
        }
        pub unsafe fn VirtualFree(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let dwFreeType = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualFree(machine, lpAddress, dwSize, dwFreeType).to_raw()
        }
        pub unsafe fn VirtualProtect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let dwSize = <u32>::from_stack(mem, esp + 8u32);
            let flNewProtect = <u32>::from_stack(mem, esp + 12u32);
            let lpflOldProtect = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            winapi::kernel32::VirtualProtect(
                machine,
                lpAddress,
                dwSize,
                flNewProtect,
                lpflOldProtect,
            )
            .to_raw()
        }
        pub unsafe fn VirtualQuery(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpAddress = <u32>::from_stack(mem, esp + 4u32);
            let lpBuffer = <Option<&mut MEMORY_BASIC_INFORMATION>>::from_stack(mem, esp + 8u32);
            let dwLength = <u32>::from_stack(mem, esp + 12u32);
            winapi::kernel32::VirtualQuery(machine, lpAddress, lpBuffer, dwLength).to_raw()
        }
        pub unsafe fn WaitForSingleObject(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hHandle = <HEVENT>::from_stack(mem, esp + 4u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 8u32);
            winapi::kernel32::WaitForSingleObject(machine, hHandle, dwMilliseconds).to_raw()
        }
        pub unsafe fn WriteConsoleA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HANDLE<()>>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleA(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteConsoleW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hConsoleOutput = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u16>>::from_stack(mem, esp + 8u32);
            let lpNumberOfCharsWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let _lpReserved = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteConsoleW(
                machine,
                hConsoleOutput,
                lpBuffer,
                lpNumberOfCharsWritten,
                _lpReserved,
            )
            .to_raw()
        }
        pub unsafe fn WriteFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hFile = <HFILE>::from_stack(mem, esp + 4u32);
            let lpBuffer = <ArrayWithSize<u8>>::from_stack(mem, esp + 8u32);
            let lpNumberOfBytesWritten = <Option<&mut u32>>::from_stack(mem, esp + 16u32);
            let lpOverlapped = <u32>::from_stack(mem, esp + 20u32);
            winapi::kernel32::WriteFile(
                machine,
                hFile,
                lpBuffer,
                lpNumberOfBytesWritten,
                lpOverlapped,
            )
            .to_raw()
        }
        pub unsafe fn lstrcmpiA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcmpiA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyA(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrcpyW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString1 = <u32>::from_stack(mem, esp + 4u32);
            let lpString2 = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            winapi::kernel32::lstrcpyW(machine, lpString1, lpString2).to_raw()
        }
        pub unsafe fn lstrlenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&str>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenA(machine, lpString).to_raw()
        }
        pub unsafe fn lstrlenW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::kernel32::lstrlenW(machine, lpString).to_raw()
        }
        pub unsafe fn retrowin32_main(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::kernel32::retrowin32_main(machine, entry_point).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 4u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::retrowin32_main(machine, entry_point));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn retrowin32_thread_main(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let entry_point = <u32>::from_stack(mem, esp + 4u32);
            let param = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::kernel32::retrowin32_thread_main(machine, entry_point, param).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 8u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::kernel32::retrowin32_thread_main(
                    machine,
                    entry_point,
                    param
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const AcquireSRWLockExclusive: Shim = Shim {
            name: "AcquireSRWLockExclusive",
            func: impls::AcquireSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const AcquireSRWLockShared: Shim = Shim {
            name: "AcquireSRWLockShared",
            func: impls::AcquireSRWLockShared,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const AddVectoredExceptionHandler: Shim = Shim {
            name: "AddVectoredExceptionHandler",
            func: impls::AddVectoredExceptionHandler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CloseHandle: Shim = Shim {
            name: "CloseHandle",
            func: impls::CloseHandle,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const CreateDirectoryA: Shim = Shim {
            name: "CreateDirectoryA",
            func: impls::CreateDirectoryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CreateEventA: Shim = Shim {
            name: "CreateEventA",
            func: impls::CreateEventA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const CreateFileA: Shim = Shim {
            name: "CreateFileA",
            func: impls::CreateFileA,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateFileW: Shim = Shim {
            name: "CreateFileW",
            func: impls::CreateFileW,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateThread: Shim = Shim {
            name: "CreateThread",
            func: impls::CreateThread,
            stack_consumed: 24u32,
            is_async: true,
        };
        pub const DeleteCriticalSection: Shim = Shim {
            name: "DeleteCriticalSection",
            func: impls::DeleteCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DeleteFileA: Shim = Shim {
            name: "DeleteFileA",
            func: impls::DeleteFileA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DisableThreadLibraryCalls: Shim = Shim {
            name: "DisableThreadLibraryCalls",
            func: impls::DisableThreadLibraryCalls,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const EnterCriticalSection: Shim = Shim {
            name: "EnterCriticalSection",
            func: impls::EnterCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ExitProcess: Shim = Shim {
            name: "ExitProcess",
            func: impls::ExitProcess,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FileTimeToSystemTime: Shim = Shim {
            name: "FileTimeToSystemTime",
            func: impls::FileTimeToSystemTime,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FindClose: Shim = Shim {
            name: "FindClose",
            func: impls::FindClose,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FindFirstFileA: Shim = Shim {
            name: "FindFirstFileA",
            func: impls::FindFirstFileA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FindNextFileA: Shim = Shim {
            name: "FindNextFileA",
            func: impls::FindNextFileA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FindResourceA: Shim = Shim {
            name: "FindResourceA",
            func: impls::FindResourceA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FindResourceW: Shim = Shim {
            name: "FindResourceW",
            func: impls::FindResourceW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FormatMessageA: Shim = Shim {
            name: "FormatMessageA",
            func: impls::FormatMessageA,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const FormatMessageW: Shim = Shim {
            name: "FormatMessageW",
            func: impls::FormatMessageW,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const FreeEnvironmentStringsA: Shim = Shim {
            name: "FreeEnvironmentStringsA",
            func: impls::FreeEnvironmentStringsA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const FreeLibrary: Shim = Shim {
            name: "FreeLibrary",
            func: impls::FreeLibrary,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetACP: Shim = Shim {
            name: "GetACP",
            func: impls::GetACP,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCPInfo: Shim = Shim {
            name: "GetCPInfo",
            func: impls::GetCPInfo,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCommandLineA: Shim = Shim {
            name: "GetCommandLineA",
            func: impls::GetCommandLineA,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCommandLineW: Shim = Shim {
            name: "GetCommandLineW",
            func: impls::GetCommandLineW,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetConsoleMode: Shim = Shim {
            name: "GetConsoleMode",
            func: impls::GetConsoleMode,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetConsoleScreenBufferInfo: Shim = Shim {
            name: "GetConsoleScreenBufferInfo",
            func: impls::GetConsoleScreenBufferInfo,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCurrentDirectoryA: Shim = Shim {
            name: "GetCurrentDirectoryA",
            func: impls::GetCurrentDirectoryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetCurrentProcessId: Shim = Shim {
            name: "GetCurrentProcessId",
            func: impls::GetCurrentProcessId,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCurrentThread: Shim = Shim {
            name: "GetCurrentThread",
            func: impls::GetCurrentThread,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetCurrentThreadId: Shim = Shim {
            name: "GetCurrentThreadId",
            func: impls::GetCurrentThreadId,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentStrings: Shim = Shim {
            name: "GetEnvironmentStrings",
            func: impls::GetEnvironmentStrings,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentStringsW: Shim = Shim {
            name: "GetEnvironmentStringsW",
            func: impls::GetEnvironmentStringsW,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetEnvironmentVariableA: Shim = Shim {
            name: "GetEnvironmentVariableA",
            func: impls::GetEnvironmentVariableA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetEnvironmentVariableW: Shim = Shim {
            name: "GetEnvironmentVariableW",
            func: impls::GetEnvironmentVariableW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetFileAttributesA: Shim = Shim {
            name: "GetFileAttributesA",
            func: impls::GetFileAttributesA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetFileInformationByHandle: Shim = Shim {
            name: "GetFileInformationByHandle",
            func: impls::GetFileInformationByHandle,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetFileSize: Shim = Shim {
            name: "GetFileSize",
            func: impls::GetFileSize,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetFileTime: Shim = Shim {
            name: "GetFileTime",
            func: impls::GetFileTime,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetFileType: Shim = Shim {
            name: "GetFileType",
            func: impls::GetFileType,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetFullPathNameA: Shim = Shim {
            name: "GetFullPathNameA",
            func: impls::GetFullPathNameA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetFullPathNameW: Shim = Shim {
            name: "GetFullPathNameW",
            func: impls::GetFullPathNameW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetLastError: Shim = Shim {
            name: "GetLastError",
            func: impls::GetLastError,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetLocalTime: Shim = Shim {
            name: "GetLocalTime",
            func: impls::GetLocalTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetModuleFileNameA: Shim = Shim {
            name: "GetModuleFileNameA",
            func: impls::GetModuleFileNameA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleFileNameW: Shim = Shim {
            name: "GetModuleFileNameW",
            func: impls::GetModuleFileNameW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleHandleA: Shim = Shim {
            name: "GetModuleHandleA",
            func: impls::GetModuleHandleA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetModuleHandleExW: Shim = Shim {
            name: "GetModuleHandleExW",
            func: impls::GetModuleHandleExW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetModuleHandleW: Shim = Shim {
            name: "GetModuleHandleW",
            func: impls::GetModuleHandleW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetPrivateProfileIntW: Shim = Shim {
            name: "GetPrivateProfileIntW",
            func: impls::GetPrivateProfileIntW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const GetPrivateProfileStringW: Shim = Shim {
            name: "GetPrivateProfileStringW",
            func: impls::GetPrivateProfileStringW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const GetProcAddress: Shim = Shim {
            name: "GetProcAddress",
            func: impls::GetProcAddress,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetProcessHeap: Shim = Shim {
            name: "GetProcessHeap",
            func: impls::GetProcessHeap,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetProfileIntW: Shim = Shim {
            name: "GetProfileIntW",
            func: impls::GetProfileIntW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetProfileStringW: Shim = Shim {
            name: "GetProfileStringW",
            func: impls::GetProfileStringW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const GetStartupInfoA: Shim = Shim {
            name: "GetStartupInfoA",
            func: impls::GetStartupInfoA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetStartupInfoW: Shim = Shim {
            name: "GetStartupInfoW",
            func: impls::GetStartupInfoW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetStdHandle: Shim = Shim {
            name: "GetStdHandle",
            func: impls::GetStdHandle,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetSystemDirectoryA: Shim = Shim {
            name: "GetSystemDirectoryA",
            func: impls::GetSystemDirectoryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetSystemTime: Shim = Shim {
            name: "GetSystemTime",
            func: impls::GetSystemTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetSystemTimeAsFileTime: Shim = Shim {
            name: "GetSystemTimeAsFileTime",
            func: impls::GetSystemTimeAsFileTime,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetTickCount: Shim = Shim {
            name: "GetTickCount",
            func: impls::GetTickCount,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetTimeZoneInformation: Shim = Shim {
            name: "GetTimeZoneInformation",
            func: impls::GetTimeZoneInformation,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetVersion: Shim = Shim {
            name: "GetVersion",
            func: impls::GetVersion,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetVersionExA: Shim = Shim {
            name: "GetVersionExA",
            func: impls::GetVersionExA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetWindowsDirectoryA: Shim = Shim {
            name: "GetWindowsDirectoryA",
            func: impls::GetWindowsDirectoryA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GlobalAlloc: Shim = Shim {
            name: "GlobalAlloc",
            func: impls::GlobalAlloc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GlobalFlags: Shim = Shim {
            name: "GlobalFlags",
            func: impls::GlobalFlags,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GlobalFree: Shim = Shim {
            name: "GlobalFree",
            func: impls::GlobalFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GlobalReAlloc: Shim = Shim {
            name: "GlobalReAlloc",
            func: impls::GlobalReAlloc,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapAlloc: Shim = Shim {
            name: "HeapAlloc",
            func: impls::HeapAlloc,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapCreate: Shim = Shim {
            name: "HeapCreate",
            func: impls::HeapCreate,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapDestroy: Shim = Shim {
            name: "HeapDestroy",
            func: impls::HeapDestroy,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const HeapFree: Shim = Shim {
            name: "HeapFree",
            func: impls::HeapFree,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const HeapReAlloc: Shim = Shim {
            name: "HeapReAlloc",
            func: impls::HeapReAlloc,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapSetInformation: Shim = Shim {
            name: "HeapSetInformation",
            func: impls::HeapSetInformation,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const HeapSize: Shim = Shim {
            name: "HeapSize",
            func: impls::HeapSize,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitOnceBeginInitialize: Shim = Shim {
            name: "InitOnceBeginInitialize",
            func: impls::InitOnceBeginInitialize,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const InitOnceComplete: Shim = Shim {
            name: "InitOnceComplete",
            func: impls::InitOnceComplete,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitializeCriticalSection: Shim = Shim {
            name: "InitializeCriticalSection",
            func: impls::InitializeCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const InitializeCriticalSectionAndSpinCount: Shim = Shim {
            name: "InitializeCriticalSectionAndSpinCount",
            func: impls::InitializeCriticalSectionAndSpinCount,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const InitializeCriticalSectionEx: Shim = Shim {
            name: "InitializeCriticalSectionEx",
            func: impls::InitializeCriticalSectionEx,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InitializeSListHead: Shim = Shim {
            name: "InitializeSListHead",
            func: impls::InitializeSListHead,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const InterlockedIncrement: Shim = Shim {
            name: "InterlockedIncrement",
            func: impls::InterlockedIncrement,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsBadReadPtr: Shim = Shim {
            name: "IsBadReadPtr",
            func: impls::IsBadReadPtr,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsBadWritePtr: Shim = Shim {
            name: "IsBadWritePtr",
            func: impls::IsBadWritePtr,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsDBCSLeadByte: Shim = Shim {
            name: "IsDBCSLeadByte",
            func: impls::IsDBCSLeadByte,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsDBCSLeadByteEx: Shim = Shim {
            name: "IsDBCSLeadByteEx",
            func: impls::IsDBCSLeadByteEx,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IsDebuggerPresent: Shim = Shim {
            name: "IsDebuggerPresent",
            func: impls::IsDebuggerPresent,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const IsProcessorFeaturePresent: Shim = Shim {
            name: "IsProcessorFeaturePresent",
            func: impls::IsProcessorFeaturePresent,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsValidCodePage: Shim = Shim {
            name: "IsValidCodePage",
            func: impls::IsValidCodePage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LeaveCriticalSection: Shim = Shim {
            name: "LeaveCriticalSection",
            func: impls::LeaveCriticalSection,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LoadLibraryA: Shim = Shim {
            name: "LoadLibraryA",
            func: impls::LoadLibraryA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LoadLibraryExW: Shim = Shim {
            name: "LoadLibraryExW",
            func: impls::LoadLibraryExW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const LoadResource: Shim = Shim {
            name: "LoadResource",
            func: impls::LoadResource,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LocalAlloc: Shim = Shim {
            name: "LocalAlloc",
            func: impls::LocalAlloc,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LocalFree: Shim = Shim {
            name: "LocalFree",
            func: impls::LocalFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LockResource: Shim = Shim {
            name: "LockResource",
            func: impls::LockResource,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const MulDiv: Shim = Shim {
            name: "MulDiv",
            func: impls::MulDiv,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const MultiByteToWideChar: Shim = Shim {
            name: "MultiByteToWideChar",
            func: impls::MultiByteToWideChar,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const NtCurrentTeb: Shim = Shim {
            name: "NtCurrentTeb",
            func: impls::NtCurrentTeb,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const OutputDebugStringA: Shim = Shim {
            name: "OutputDebugStringA",
            func: impls::OutputDebugStringA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const QueryPerformanceCounter: Shim = Shim {
            name: "QueryPerformanceCounter",
            func: impls::QueryPerformanceCounter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const QueryPerformanceFrequency: Shim = Shim {
            name: "QueryPerformanceFrequency",
            func: impls::QueryPerformanceFrequency,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReadFile: Shim = Shim {
            name: "ReadFile",
            func: impls::ReadFile,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const ReleaseSRWLockExclusive: Shim = Shim {
            name: "ReleaseSRWLockExclusive",
            func: impls::ReleaseSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReleaseSRWLockShared: Shim = Shim {
            name: "ReleaseSRWLockShared",
            func: impls::ReleaseSRWLockShared,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RemoveDirectoryA: Shim = Shim {
            name: "RemoveDirectoryA",
            func: impls::RemoveDirectoryA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ResumeThread: Shim = Shim {
            name: "ResumeThread",
            func: impls::ResumeThread,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetConsoleCtrlHandler: Shim = Shim {
            name: "SetConsoleCtrlHandler",
            func: impls::SetConsoleCtrlHandler,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetEndOfFile: Shim = Shim {
            name: "SetEndOfFile",
            func: impls::SetEndOfFile,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetEnvironmentVariableA: Shim = Shim {
            name: "SetEnvironmentVariableA",
            func: impls::SetEnvironmentVariableA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetEvent: Shim = Shim {
            name: "SetEvent",
            func: impls::SetEvent,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetFileAttributesA: Shim = Shim {
            name: "SetFileAttributesA",
            func: impls::SetFileAttributesA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetFilePointer: Shim = Shim {
            name: "SetFilePointer",
            func: impls::SetFilePointer,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetFileTime: Shim = Shim {
            name: "SetFileTime",
            func: impls::SetFileTime,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetHandleCount: Shim = Shim {
            name: "SetHandleCount",
            func: impls::SetHandleCount,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetLastError: Shim = Shim {
            name: "SetLastError",
            func: impls::SetLastError,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetPriorityClass: Shim = Shim {
            name: "SetPriorityClass",
            func: impls::SetPriorityClass,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetStdHandle: Shim = Shim {
            name: "SetStdHandle",
            func: impls::SetStdHandle,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadDescription: Shim = Shim {
            name: "SetThreadDescription",
            func: impls::SetThreadDescription,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadPriority: Shim = Shim {
            name: "SetThreadPriority",
            func: impls::SetThreadPriority,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetThreadStackGuarantee: Shim = Shim {
            name: "SetThreadStackGuarantee",
            func: impls::SetThreadStackGuarantee,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetUnhandledExceptionFilter: Shim = Shim {
            name: "SetUnhandledExceptionFilter",
            func: impls::SetUnhandledExceptionFilter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SizeofResource: Shim = Shim {
            name: "SizeofResource",
            func: impls::SizeofResource,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const Sleep: Shim = Shim {
            name: "Sleep",
            func: impls::Sleep,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const SystemTimeToFileTime: Shim = Shim {
            name: "SystemTimeToFileTime",
            func: impls::SystemTimeToFileTime,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TlsAlloc: Shim = Shim {
            name: "TlsAlloc",
            func: impls::TlsAlloc,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const TlsFree: Shim = Shim {
            name: "TlsFree",
            func: impls::TlsFree,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const TlsGetValue: Shim = Shim {
            name: "TlsGetValue",
            func: impls::TlsGetValue,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const TlsSetValue: Shim = Shim {
            name: "TlsSetValue",
            func: impls::TlsSetValue,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const TryAcquireSRWLockExclusive: Shim = Shim {
            name: "TryAcquireSRWLockExclusive",
            func: impls::TryAcquireSRWLockExclusive,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const UnhandledExceptionFilter: Shim = Shim {
            name: "UnhandledExceptionFilter",
            func: impls::UnhandledExceptionFilter,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const VirtualAlloc: Shim = Shim {
            name: "VirtualAlloc",
            func: impls::VirtualAlloc,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const VirtualFree: Shim = Shim {
            name: "VirtualFree",
            func: impls::VirtualFree,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const VirtualProtect: Shim = Shim {
            name: "VirtualProtect",
            func: impls::VirtualProtect,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const VirtualQuery: Shim = Shim {
            name: "VirtualQuery",
            func: impls::VirtualQuery,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const WaitForSingleObject: Shim = Shim {
            name: "WaitForSingleObject",
            func: impls::WaitForSingleObject,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const WriteConsoleA: Shim = Shim {
            name: "WriteConsoleA",
            func: impls::WriteConsoleA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const WriteConsoleW: Shim = Shim {
            name: "WriteConsoleW",
            func: impls::WriteConsoleW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const WriteFile: Shim = Shim {
            name: "WriteFile",
            func: impls::WriteFile,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const lstrcmpiA: Shim = Shim {
            name: "lstrcmpiA",
            func: impls::lstrcmpiA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrcpyA: Shim = Shim {
            name: "lstrcpyA",
            func: impls::lstrcpyA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrcpyW: Shim = Shim {
            name: "lstrcpyW",
            func: impls::lstrcpyW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const lstrlenA: Shim = Shim {
            name: "lstrlenA",
            func: impls::lstrlenA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const lstrlenW: Shim = Shim {
            name: "lstrlenW",
            func: impls::lstrlenW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const retrowin32_main: Shim = Shim {
            name: "retrowin32_main",
            func: impls::retrowin32_main,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const retrowin32_thread_main: Shim = Shim {
            name: "retrowin32_thread_main",
            func: impls::retrowin32_thread_main,
            stack_consumed: 8u32,
            is_async: true,
        };
    }
    const SHIMS: [Shim; 151usize] = [
        shims::AcquireSRWLockExclusive,
        shims::AcquireSRWLockShared,
        shims::AddVectoredExceptionHandler,
        shims::CloseHandle,
        shims::CreateDirectoryA,
        shims::CreateEventA,
        shims::CreateFileA,
        shims::CreateFileW,
        shims::CreateThread,
        shims::DeleteCriticalSection,
        shims::DeleteFileA,
        shims::DisableThreadLibraryCalls,
        shims::EnterCriticalSection,
        shims::ExitProcess,
        shims::FileTimeToSystemTime,
        shims::FindClose,
        shims::FindFirstFileA,
        shims::FindNextFileA,
        shims::FindResourceA,
        shims::FindResourceW,
        shims::FormatMessageA,
        shims::FormatMessageW,
        shims::FreeEnvironmentStringsA,
        shims::FreeLibrary,
        shims::GetACP,
        shims::GetCPInfo,
        shims::GetCommandLineA,
        shims::GetCommandLineW,
        shims::GetConsoleMode,
        shims::GetConsoleScreenBufferInfo,
        shims::GetCurrentDirectoryA,
        shims::GetCurrentProcessId,
        shims::GetCurrentThread,
        shims::GetCurrentThreadId,
        shims::GetEnvironmentStrings,
        shims::GetEnvironmentStringsW,
        shims::GetEnvironmentVariableA,
        shims::GetEnvironmentVariableW,
        shims::GetFileAttributesA,
        shims::GetFileInformationByHandle,
        shims::GetFileSize,
        shims::GetFileTime,
        shims::GetFileType,
        shims::GetFullPathNameA,
        shims::GetFullPathNameW,
        shims::GetLastError,
        shims::GetLocalTime,
        shims::GetModuleFileNameA,
        shims::GetModuleFileNameW,
        shims::GetModuleHandleA,
        shims::GetModuleHandleExW,
        shims::GetModuleHandleW,
        shims::GetPrivateProfileIntW,
        shims::GetPrivateProfileStringW,
        shims::GetProcAddress,
        shims::GetProcessHeap,
        shims::GetProfileIntW,
        shims::GetProfileStringW,
        shims::GetStartupInfoA,
        shims::GetStartupInfoW,
        shims::GetStdHandle,
        shims::GetSystemDirectoryA,
        shims::GetSystemTime,
        shims::GetSystemTimeAsFileTime,
        shims::GetTickCount,
        shims::GetTimeZoneInformation,
        shims::GetVersion,
        shims::GetVersionExA,
        shims::GetWindowsDirectoryA,
        shims::GlobalAlloc,
        shims::GlobalFlags,
        shims::GlobalFree,
        shims::GlobalReAlloc,
        shims::HeapAlloc,
        shims::HeapCreate,
        shims::HeapDestroy,
        shims::HeapFree,
        shims::HeapReAlloc,
        shims::HeapSetInformation,
        shims::HeapSize,
        shims::InitOnceBeginInitialize,
        shims::InitOnceComplete,
        shims::InitializeCriticalSection,
        shims::InitializeCriticalSectionAndSpinCount,
        shims::InitializeCriticalSectionEx,
        shims::InitializeSListHead,
        shims::InterlockedIncrement,
        shims::IsBadReadPtr,
        shims::IsBadWritePtr,
        shims::IsDBCSLeadByte,
        shims::IsDBCSLeadByteEx,
        shims::IsDebuggerPresent,
        shims::IsProcessorFeaturePresent,
        shims::IsValidCodePage,
        shims::LeaveCriticalSection,
        shims::LoadLibraryA,
        shims::LoadLibraryExW,
        shims::LoadResource,
        shims::LocalAlloc,
        shims::LocalFree,
        shims::LockResource,
        shims::MulDiv,
        shims::MultiByteToWideChar,
        shims::NtCurrentTeb,
        shims::OutputDebugStringA,
        shims::QueryPerformanceCounter,
        shims::QueryPerformanceFrequency,
        shims::ReadFile,
        shims::ReleaseSRWLockExclusive,
        shims::ReleaseSRWLockShared,
        shims::RemoveDirectoryA,
        shims::ResumeThread,
        shims::SetConsoleCtrlHandler,
        shims::SetEndOfFile,
        shims::SetEnvironmentVariableA,
        shims::SetEvent,
        shims::SetFileAttributesA,
        shims::SetFilePointer,
        shims::SetFileTime,
        shims::SetHandleCount,
        shims::SetLastError,
        shims::SetPriorityClass,
        shims::SetStdHandle,
        shims::SetThreadDescription,
        shims::SetThreadPriority,
        shims::SetThreadStackGuarantee,
        shims::SetUnhandledExceptionFilter,
        shims::SizeofResource,
        shims::Sleep,
        shims::SystemTimeToFileTime,
        shims::TlsAlloc,
        shims::TlsFree,
        shims::TlsGetValue,
        shims::TlsSetValue,
        shims::TryAcquireSRWLockExclusive,
        shims::UnhandledExceptionFilter,
        shims::VirtualAlloc,
        shims::VirtualFree,
        shims::VirtualProtect,
        shims::VirtualQuery,
        shims::WaitForSingleObject,
        shims::WriteConsoleA,
        shims::WriteConsoleW,
        shims::WriteFile,
        shims::lstrcmpiA,
        shims::lstrcpyA,
        shims::lstrcpyW,
        shims::lstrlenA,
        shims::lstrlenW,
        shims::retrowin32_main,
        shims::retrowin32_thread_main,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/kernel32.dll"),
    };
}
pub mod ntdll {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ntdll::*;
        pub unsafe fn NtReadFile(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let FileHandle = <HFILE>::from_stack(mem, esp + 4u32);
            let Event = <u32>::from_stack(mem, esp + 8u32);
            let ApcRoutine = <u32>::from_stack(mem, esp + 12u32);
            let ApcContext = <u32>::from_stack(mem, esp + 16u32);
            let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, esp + 20u32);
            let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, esp + 24u32);
            let ByteOffset = <Option<&mut u64>>::from_stack(mem, esp + 32u32);
            let Key = <u32>::from_stack(mem, esp + 36u32);
            winapi::ntdll::NtReadFile(
                machine,
                FileHandle,
                Event,
                ApcRoutine,
                ApcContext,
                IoStatusBlock,
                Buffer,
                ByteOffset,
                Key,
            )
            .to_raw()
        }
        pub unsafe fn RtlExitUserProcess(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let exit_code = <u32>::from_stack(mem, esp + 4u32);
            winapi::ntdll::RtlExitUserProcess(machine, exit_code).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const NtReadFile: Shim = Shim {
            name: "NtReadFile",
            func: impls::NtReadFile,
            stack_consumed: 36u32,
            is_async: false,
        };
        pub const RtlExitUserProcess: Shim = Shim {
            name: "RtlExitUserProcess",
            func: impls::RtlExitUserProcess,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 2usize] = [shims::NtReadFile, shims::RtlExitUserProcess];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ntdll.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ntdll.dll"),
    };
}
pub mod ole32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ole32::*;
        pub unsafe fn OleInitialize(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _pvReserved = <u32>::from_stack(mem, esp + 4u32);
            winapi::ole32::OleInitialize(machine, _pvReserved).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const OleInitialize: Shim = Shim {
            name: "OleInitialize",
            func: impls::OleInitialize,
            stack_consumed: 4u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::OleInitialize];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ole32.dll"),
    };
}
pub mod oleaut32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::oleaut32::*;
    }
    mod shims {
        use super::impls;
        use super::Shim;
    }
    const SHIMS: [Shim; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/oleaut32.dll"),
    };
}
pub mod retrowin32_test {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::retrowin32_test::*;
        pub unsafe fn retrowin32_test_callback1(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let data = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data)
                            .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 8u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::retrowin32_test::retrowin32_test_callback1(
                    machine, func, data
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const retrowin32_test_callback1: Shim = Shim {
            name: "retrowin32_test_callback1",
            func: impls::retrowin32_test_callback1,
            stack_consumed: 8u32,
            is_async: true,
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::retrowin32_test_callback1];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "retrowin32_test.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/retrowin32_test.dll"),
    };
}
pub mod ucrtbase {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::ucrtbase::*;
        pub unsafe fn __dllonexit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let func = <u32>::from_stack(mem, esp + 4u32);
            let d = <u32>::from_stack(mem, esp + 8u32);
            let f = <u32>::from_stack(mem, esp + 12u32);
            winapi::ucrtbase::__dllonexit(machine, func, d, f).to_raw()
        }
        pub unsafe fn __p___argc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argc(machine).to_raw()
        }
        pub unsafe fn __p___argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p___argv(machine).to_raw()
        }
        pub unsafe fn __p__commode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__commode(machine).to_raw()
        }
        pub unsafe fn __p__fmode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::__p__fmode(machine).to_raw()
        }
        pub unsafe fn __set_app_type(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::__set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _configthreadlocale(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let per_thread_locale_type = <i32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_configthreadlocale(machine, per_thread_locale_type).to_raw()
        }
        pub unsafe fn _configure_narrow_argv(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_configure_narrow_argv(machine, _mode).to_raw()
        }
        pub unsafe fn _controlfp_s(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _currentControl = <u32>::from_stack(mem, esp + 4u32);
            let _newControl = <u32>::from_stack(mem, esp + 8u32);
            let _mask = <u32>::from_stack(mem, esp + 12u32);
            winapi::ucrtbase::_controlfp_s(machine, _currentControl, _newControl, _mask).to_raw()
        }
        pub unsafe fn _crt_atexit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _function = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_crt_atexit(machine, _function).to_raw()
        }
        pub unsafe fn _get_initial_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_get_initial_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initialize_narrow_environment(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::ucrtbase::_initialize_narrow_environment(machine).to_raw()
        }
        pub unsafe fn _initterm(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::ucrtbase::_initterm(machine, start, end).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 0u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::ucrtbase::_initterm(machine, start, end));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn _initterm_e(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let start = <u32>::from_stack(mem, esp + 4u32);
            let end = <u32>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::ucrtbase::_initterm_e(machine, start, end).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 0u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::ucrtbase::_initterm_e(machine, start, end));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn _lock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_lock(machine, locknum).to_raw()
        }
        pub unsafe fn _set_app_type(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _app_type = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_app_type(machine, _app_type).to_raw()
        }
        pub unsafe fn _set_fmode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let _mode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_fmode(machine, _mode).to_raw()
        }
        pub unsafe fn _set_new_mode(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let newhandlermode = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_set_new_mode(machine, newhandlermode).to_raw()
        }
        pub unsafe fn _unlock(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let locknum = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::_unlock(machine, locknum).to_raw()
        }
        pub unsafe fn exit(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let status = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::exit(machine, status).to_raw()
        }
        pub unsafe fn free(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let ptr = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::free(machine, ptr).to_raw()
        }
        pub unsafe fn malloc(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let size = <u32>::from_stack(mem, esp + 4u32);
            winapi::ucrtbase::malloc(machine, size).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const __dllonexit: Shim = Shim {
            name: "__dllonexit",
            func: impls::__dllonexit,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __p___argc: Shim = Shim {
            name: "__p___argc",
            func: impls::__p___argc,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __p___argv: Shim = Shim {
            name: "__p___argv",
            func: impls::__p___argv,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __p__commode: Shim = Shim {
            name: "__p__commode",
            func: impls::__p__commode,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __p__fmode: Shim = Shim {
            name: "__p__fmode",
            func: impls::__p__fmode,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const __set_app_type: Shim = Shim {
            name: "__set_app_type",
            func: impls::__set_app_type,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _configthreadlocale: Shim = Shim {
            name: "_configthreadlocale",
            func: impls::_configthreadlocale,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _configure_narrow_argv: Shim = Shim {
            name: "_configure_narrow_argv",
            func: impls::_configure_narrow_argv,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _controlfp_s: Shim = Shim {
            name: "_controlfp_s",
            func: impls::_controlfp_s,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _crt_atexit: Shim = Shim {
            name: "_crt_atexit",
            func: impls::_crt_atexit,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _get_initial_narrow_environment: Shim = Shim {
            name: "_get_initial_narrow_environment",
            func: impls::_get_initial_narrow_environment,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _initialize_narrow_environment: Shim = Shim {
            name: "_initialize_narrow_environment",
            func: impls::_initialize_narrow_environment,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _initterm: Shim = Shim {
            name: "_initterm",
            func: impls::_initterm,
            stack_consumed: 0u32,
            is_async: true,
        };
        pub const _initterm_e: Shim = Shim {
            name: "_initterm_e",
            func: impls::_initterm_e,
            stack_consumed: 0u32,
            is_async: true,
        };
        pub const _lock: Shim = Shim {
            name: "_lock",
            func: impls::_lock,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _set_app_type: Shim = Shim {
            name: "_set_app_type",
            func: impls::_set_app_type,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _set_fmode: Shim = Shim {
            name: "_set_fmode",
            func: impls::_set_fmode,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _set_new_mode: Shim = Shim {
            name: "_set_new_mode",
            func: impls::_set_new_mode,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const _unlock: Shim = Shim {
            name: "_unlock",
            func: impls::_unlock,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const exit: Shim = Shim {
            name: "exit",
            func: impls::exit,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const free: Shim = Shim {
            name: "free",
            func: impls::free,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const malloc: Shim = Shim {
            name: "malloc",
            func: impls::malloc,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 22usize] = [
        shims::__dllonexit,
        shims::__p___argc,
        shims::__p___argv,
        shims::__p__commode,
        shims::__p__fmode,
        shims::__set_app_type,
        shims::_configthreadlocale,
        shims::_configure_narrow_argv,
        shims::_controlfp_s,
        shims::_crt_atexit,
        shims::_get_initial_narrow_environment,
        shims::_initialize_narrow_environment,
        shims::_initterm,
        shims::_initterm_e,
        shims::_lock,
        shims::_set_app_type,
        shims::_set_fmode,
        shims::_set_new_mode,
        shims::_unlock,
        shims::exit,
        shims::free,
        shims::malloc,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ucrtbase.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/ucrtbase.dll"),
    };
}
pub mod vcruntime140 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::vcruntime140::*;
        pub unsafe fn _CxxThrowException(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let pExceptionObject = <u32>::from_stack(mem, esp + 4u32);
            let pThrowInfo = <u32>::from_stack(mem, esp + 8u32);
            winapi::vcruntime140::_CxxThrowException(machine, pExceptionObject, pThrowInfo).to_raw()
        }
        pub unsafe fn memcmp(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lhs = <u32>::from_stack(mem, esp + 4u32);
            let rhs = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcmp(machine, lhs, rhs, len).to_raw()
        }
        pub unsafe fn memcpy(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let src = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memcpy(machine, dst, src, len).to_raw()
        }
        pub unsafe fn memset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dst = <u32>::from_stack(mem, esp + 4u32);
            let val = <u32>::from_stack(mem, esp + 8u32);
            let len = <u32>::from_stack(mem, esp + 12u32);
            winapi::vcruntime140::memset(machine, dst, val, len).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const _CxxThrowException: Shim = Shim {
            name: "_CxxThrowException",
            func: impls::_CxxThrowException,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memcmp: Shim = Shim {
            name: "memcmp",
            func: impls::memcmp,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memcpy: Shim = Shim {
            name: "memcpy",
            func: impls::memcpy,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const memset: Shim = Shim {
            name: "memset",
            func: impls::memset,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 4usize] = [
        shims::_CxxThrowException,
        shims::memcmp,
        shims::memcpy,
        shims::memset,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "vcruntime140.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/vcruntime140.dll"),
    };
}
pub mod version {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::version::*;
        pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lptstrFilename = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpdwHandle = <Option<&mut u32>>::from_stack(mem, esp + 8u32);
            winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const GetFileVersionInfoSizeA: Shim = Shim {
            name: "GetFileVersionInfoSizeA",
            func: impls::GetFileVersionInfoSizeA,
            stack_consumed: 8u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::GetFileVersionInfoSizeA];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "version.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/version.dll"),
    };
}
pub mod user32 {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::user32::*;
        pub unsafe fn AdjustWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::AdjustWindowRect(machine, lpRect, dwStyle, bMenu).to_raw()
        }
        pub unsafe fn AdjustWindowRectEx(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 8u32);
            let bMenu = <bool>::from_stack(mem, esp + 12u32);
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 16u32);
            winapi::user32::AdjustWindowRectEx(machine, lpRect, dwStyle, bMenu, dwExStyle).to_raw()
        }
        pub unsafe fn AppendMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uFlags = <u32>::from_stack(mem, esp + 8u32);
            let uIDNewItem = <u32>::from_stack(mem, esp + 12u32);
            let lpNewItem = <Option<&str>>::from_stack(mem, esp + 16u32);
            winapi::user32::AppendMenuA(machine, hMenu, uFlags, uIDNewItem, lpNewItem).to_raw()
        }
        pub unsafe fn BeginPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&mut PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::BeginPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn CheckMenuItem(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let uIDCheckItem = <u32>::from_stack(mem, esp + 8u32);
            let uCheck = <u32>::from_stack(mem, esp + 12u32);
            winapi::user32::CheckMenuItem(machine, hMenu, uIDCheckItem, uCheck).to_raw()
        }
        pub unsafe fn ClientToScreen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPoint = <Option<&mut POINT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ClientToScreen(machine, hWnd, lpPoint).to_raw()
        }
        pub unsafe fn CreateCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInst = <u32>::from_stack(mem, esp + 4u32);
            let xHotSpot = <u32>::from_stack(mem, esp + 8u32);
            let yHotSpot = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let pvANDPlane = <u32>::from_stack(mem, esp + 24u32);
            let pvXORPlane = <u32>::from_stack(mem, esp + 28u32);
            winapi::user32::CreateCursor(
                machine, hInst, xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane,
            )
            .to_raw()
        }
        pub unsafe fn CreateWindowExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, str>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::CreateWindowExA(
                        machine,
                        dwExStyle,
                        lpClassName,
                        lpWindowName,
                        dwStyle,
                        X,
                        Y,
                        nWidth,
                        nHeight,
                        hWndParent,
                        hMenu,
                        hInstance,
                        lpParam,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 48u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::CreateWindowExA(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn CreateWindowExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let dwExStyle = <Result<WindowStyleEx, u32>>::from_stack(mem, esp + 4u32);
            let lpClassName = <CreateWindowClassName<'_, Str16>>::from_stack(mem, esp + 8u32);
            let lpWindowName = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let dwStyle = <Result<WindowStyle, u32>>::from_stack(mem, esp + 16u32);
            let X = <u32>::from_stack(mem, esp + 20u32);
            let Y = <u32>::from_stack(mem, esp + 24u32);
            let nWidth = <u32>::from_stack(mem, esp + 28u32);
            let nHeight = <u32>::from_stack(mem, esp + 32u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 36u32);
            let hMenu = <u32>::from_stack(mem, esp + 40u32);
            let hInstance = <u32>::from_stack(mem, esp + 44u32);
            let lpParam = <u32>::from_stack(mem, esp + 48u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::CreateWindowExW(
                        machine,
                        dwExStyle,
                        lpClassName,
                        lpWindowName,
                        dwStyle,
                        X,
                        Y,
                        nWidth,
                        nHeight,
                        hWndParent,
                        hMenu,
                        hInstance,
                        lpParam,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 48u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::CreateWindowExW(
                    machine,
                    dwExStyle,
                    lpClassName,
                    lpWindowName,
                    dwStyle,
                    X,
                    Y,
                    nWidth,
                    nHeight,
                    hWndParent,
                    hMenu,
                    hInstance,
                    lpParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DefWindowProcA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::DefWindowProcA(machine, hWnd, msg, wParam, lParam).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 16u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DefWindowProcA(
                    machine, hWnd, msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DefWindowProcW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::DefWindowProcW(machine, hWnd, msg, wParam, lParam).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 16u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DefWindowProcW(
                    machine, hWnd, msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DestroyWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::DestroyWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn DialogBoxIndirectParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let hDialogTemplate = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxIndirectParamA(
                machine,
                hInstance,
                hDialogTemplate,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DialogBoxParamA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTemplateName = <u32>::from_stack(mem, esp + 8u32);
            let hWndParent = <HWND>::from_stack(mem, esp + 12u32);
            let lpDialogFunc = <u32>::from_stack(mem, esp + 16u32);
            let dwInitParam = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DialogBoxParamA(
                machine,
                hInstance,
                lpTemplateName,
                hWndParent,
                lpDialogFunc,
                dwInitParam,
            )
            .to_raw()
        }
        pub unsafe fn DispatchMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::DispatchMessageA(machine, lpMsg).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 4u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DispatchMessageA(machine, lpMsg));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DispatchMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::DispatchMessageW(machine, lpMsg).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 4u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::DispatchMessageW(machine, lpMsg));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn DrawTextW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let nCount = <i32>::from_stack(mem, esp + 12u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 16u32);
            let uFormat = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::DrawTextW(machine, hDC, lpString, nCount, lpRect, uFormat).to_raw()
        }
        pub unsafe fn EndPaint(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpPaint = <Option<&PAINTSTRUCT>>::from_stack(mem, esp + 8u32);
            winapi::user32::EndPaint(machine, hWnd, lpPaint).to_raw()
        }
        pub unsafe fn FillRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <BrushOrColor>::from_stack(mem, esp + 12u32);
            winapi::user32::FillRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn FindWindowA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpClassName = <Option<&str>>::from_stack(mem, esp + 4u32);
            let lpWindowName = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::FindWindowA(machine, lpClassName, lpWindowName).to_raw()
        }
        pub unsafe fn FrameRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hDC = <HDC>::from_stack(mem, esp + 4u32);
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let hbr = <HBRUSH>::from_stack(mem, esp + 12u32);
            winapi::user32::FrameRect(machine, hDC, lprc, hbr).to_raw()
        }
        pub unsafe fn GetActiveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetActiveWindow(machine).to_raw()
        }
        pub unsafe fn GetClientRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetClientRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn GetDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetDesktopWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetDesktopWindow(machine).to_raw()
        }
        pub unsafe fn GetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetFocus(machine).to_raw()
        }
        pub unsafe fn GetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetForegroundWindow(machine).to_raw()
        }
        pub unsafe fn GetKeyState(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nVirtKey = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::GetKeyState(machine, nVirtKey).to_raw()
        }
        pub unsafe fn GetLastActivePopup(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::GetLastActivePopup(machine).to_raw()
        }
        pub unsafe fn GetMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::GetMessageA(
                        machine,
                        lpMsg,
                        hWnd,
                        wMsgFilterMin,
                        wMsgFilterMax,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 16u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::GetMessageA(
                    machine,
                    lpMsg,
                    hWnd,
                    wMsgFilterMin,
                    wMsgFilterMax
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn GetMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::GetMessageW(
                        machine,
                        lpMsg,
                        hWnd,
                        wMsgFilterMin,
                        wMsgFilterMax,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 16u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::GetMessageW(
                    machine,
                    lpMsg,
                    hWnd,
                    wMsgFilterMin,
                    wMsgFilterMax
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn GetSystemMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let bRevert = <bool>::from_stack(mem, esp + 8u32);
            winapi::user32::GetSystemMenu(machine, hWnd, bRevert).to_raw()
        }
        pub unsafe fn GetSystemMetrics(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nIndex = <Result<SystemMetric, u32>>::from_stack(mem, esp + 4u32);
            winapi::user32::GetSystemMetrics(machine, nIndex).to_raw()
        }
        pub unsafe fn GetWindowDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::GetWindowDC(machine, hWnd).to_raw()
        }
        pub unsafe fn GetWindowLongA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIndex = <i32>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowLongA(machine, hWnd, nIndex).to_raw()
        }
        pub unsafe fn GetWindowRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&mut RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::GetWindowRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn IntersectRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprcDst = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let lprcSrc1 = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let lprcSrc2 = <Option<&RECT>>::from_stack(mem, esp + 12u32);
            winapi::user32::IntersectRect(machine, lprcDst, lprcSrc1, lprcSrc2).to_raw()
        }
        pub unsafe fn InvalidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRect(machine, hWnd, lpRect, bErase).to_raw()
        }
        pub unsafe fn InvalidateRgn(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hRgn = <HRGN>::from_stack(mem, esp + 8u32);
            let bErase = <bool>::from_stack(mem, esp + 12u32);
            winapi::user32::InvalidateRgn(machine, hWnd, hRgn, bErase).to_raw()
        }
        pub unsafe fn IsIconic(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::IsIconic(machine, hwnd).to_raw()
        }
        pub unsafe fn IsRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::IsRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn LoadAcceleratorsW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpTableName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadAcceleratorsW(machine, hInstance, lpTableName).to_raw()
        }
        pub unsafe fn LoadBitmapA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <HINSTANCE>::from_stack(mem, esp + 4u32);
            let lpBitmapName = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadBitmapA(machine, hInstance, lpBitmapName).to_raw()
        }
        pub unsafe fn LoadCursorA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorA(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadCursorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpCursorName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadCursorW(machine, hInstance, lpCursorName).to_raw()
        }
        pub unsafe fn LoadIconA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconA(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadIconW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpIconName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadIconW(machine, hInstance, lpIconName).to_raw()
        }
        pub unsafe fn LoadImageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&str>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageA(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadImageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let name = <ResourceKey<&Str16>>::from_stack(mem, esp + 8u32);
            let typ = <u32>::from_stack(mem, esp + 12u32);
            let cx = <u32>::from_stack(mem, esp + 16u32);
            let cy = <u32>::from_stack(mem, esp + 20u32);
            let fuLoad = <u32>::from_stack(mem, esp + 24u32);
            winapi::user32::LoadImageW(machine, hInstance, name, typ, cx, cy, fuLoad).to_raw()
        }
        pub unsafe fn LoadMenuA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuA(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadMenuW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let lpMenuName = <u32>::from_stack(mem, esp + 8u32);
            winapi::user32::LoadMenuW(machine, hInstance, lpMenuName).to_raw()
        }
        pub unsafe fn LoadStringA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringA(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn LoadStringW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hInstance = <u32>::from_stack(mem, esp + 4u32);
            let uID = <u32>::from_stack(mem, esp + 8u32);
            let lpBuffer = <u32>::from_stack(mem, esp + 12u32);
            let cchBufferMax = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::LoadStringW(machine, hInstance, uID, lpBuffer, cchBufferMax).to_raw()
        }
        pub unsafe fn MapWindowPoints(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWndFrom = <HWND>::from_stack(mem, esp + 4u32);
            let hWndTo = <HWND>::from_stack(mem, esp + 8u32);
            let lpPoints = <ArrayWithSize<POINT>>::from_stack(mem, esp + 12u32);
            winapi::user32::MapWindowPoints(machine, hWndFrom, hWndTo, lpPoints).to_raw()
        }
        pub unsafe fn MessageBoxA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&str>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&str>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxA(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MessageBoxW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpText = <Option<&Str16>>::from_stack(mem, esp + 8u32);
            let lpCaption = <Option<&Str16>>::from_stack(mem, esp + 12u32);
            let uType = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::MessageBoxW(machine, hWnd, lpText, lpCaption, uType).to_raw()
        }
        pub unsafe fn MoveWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let X = <u32>::from_stack(mem, esp + 8u32);
            let Y = <u32>::from_stack(mem, esp + 12u32);
            let nWidth = <u32>::from_stack(mem, esp + 16u32);
            let nHeight = <u32>::from_stack(mem, esp + 20u32);
            let bRepaint = <bool>::from_stack(mem, esp + 24u32);
            winapi::user32::MoveWindow(machine, hWnd, X, Y, nWidth, nHeight, bRepaint).to_raw()
        }
        pub unsafe fn MsgWaitForMultipleObjects(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nCount = <u32>::from_stack(mem, esp + 4u32);
            let pHandles = <u32>::from_stack(mem, esp + 8u32);
            let fWaitAll = <bool>::from_stack(mem, esp + 12u32);
            let dwMilliseconds = <u32>::from_stack(mem, esp + 16u32);
            let dwWakeMask = <u32>::from_stack(mem, esp + 20u32);
            winapi::user32::MsgWaitForMultipleObjects(
                machine,
                nCount,
                pHandles,
                fWaitAll,
                dwMilliseconds,
                dwWakeMask,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageA(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PeekMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&mut MSG>>::from_stack(mem, esp + 4u32);
            let hWnd = <HWND>::from_stack(mem, esp + 8u32);
            let wMsgFilterMin = <u32>::from_stack(mem, esp + 12u32);
            let wMsgFilterMax = <u32>::from_stack(mem, esp + 16u32);
            let wRemoveMsg = <Result<RemoveMsg, u32>>::from_stack(mem, esp + 20u32);
            winapi::user32::PeekMessageW(
                machine,
                lpMsg,
                hWnd,
                wMsgFilterMin,
                wMsgFilterMax,
                wRemoveMsg,
            )
            .to_raw()
        }
        pub unsafe fn PostMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <u32>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::PostMessageW(machine, hWnd, Msg, wParam, lParam).to_raw()
        }
        pub unsafe fn PostQuitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let nExitCode = <i32>::from_stack(mem, esp + 4u32);
            winapi::user32::PostQuitMessage(machine, nExitCode).to_raw()
        }
        pub unsafe fn PtInRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&RECT>>::from_stack(mem, esp + 4u32);
            let pt = <POINT>::from_stack(mem, esp + 8u32);
            winapi::user32::PtInRect(machine, lprc, pt).to_raw()
        }
        pub unsafe fn RegisterClassA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassA(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterClassExA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExA(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassExW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClassEx = <Option<&WNDCLASSEXW>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassExW(machine, lpWndClassEx).to_raw()
        }
        pub unsafe fn RegisterClassW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpWndClass = <Option<&WNDCLASSA>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterClassW(machine, lpWndClass).to_raw()
        }
        pub unsafe fn RegisterWindowMessageW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpString = <Option<&Str16>>::from_stack(mem, esp + 4u32);
            winapi::user32::RegisterWindowMessageW(machine, lpString).to_raw()
        }
        pub unsafe fn ReleaseCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::ReleaseCapture(machine).to_raw()
        }
        pub unsafe fn ReleaseDC(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            let hdc = <HDC>::from_stack(mem, esp + 8u32);
            winapi::user32::ReleaseDC(machine, hwnd, hdc).to_raw()
        }
        pub unsafe fn SendMessageA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let Msg = <Result<WM, u32>>::from_stack(mem, esp + 8u32);
            let wParam = <u32>::from_stack(mem, esp + 12u32);
            let lParam = <u32>::from_stack(mem, esp + 16u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result =
                        winapi::user32::SendMessageA(machine, hWnd, Msg, wParam, lParam).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 16u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::SendMessageA(
                    machine, hWnd, Msg, wParam, lParam
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn SetCapture(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCapture(machine, hwnd).to_raw()
        }
        pub unsafe fn SetCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hCursor = <u32>::from_stack(mem, esp + 4u32);
            winapi::user32::SetCursor(machine, hCursor).to_raw()
        }
        pub unsafe fn SetFocus(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetFocus(machine, hWnd).to_raw()
        }
        pub unsafe fn SetForegroundWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            winapi::user32::SetForegroundWindow(machine, hWnd).to_raw()
        }
        pub unsafe fn SetMenu(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hMenu = <HMENU>::from_stack(mem, esp + 8u32);
            winapi::user32::SetMenu(machine, hWnd, hMenu).to_raw()
        }
        pub unsafe fn SetMenuItemInfoA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hMenu = <HMENU>::from_stack(mem, esp + 4u32);
            let item = <u32>::from_stack(mem, esp + 8u32);
            let fByPosition = <bool>::from_stack(mem, esp + 12u32);
            let lpmii = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetMenuItemInfoA(machine, hMenu, item, fByPosition, lpmii).to_raw()
        }
        pub unsafe fn SetRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            let xLeft = <i32>::from_stack(mem, esp + 8u32);
            let yTop = <i32>::from_stack(mem, esp + 12u32);
            let xRight = <i32>::from_stack(mem, esp + 16u32);
            let yBottom = <i32>::from_stack(mem, esp + 20u32);
            winapi::user32::SetRect(machine, lprc, xLeft, yTop, xRight, yBottom).to_raw()
        }
        pub unsafe fn SetRectEmpty(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lprc = <Option<&mut RECT>>::from_stack(mem, esp + 4u32);
            winapi::user32::SetRectEmpty(machine, lprc).to_raw()
        }
        pub unsafe fn SetTimer(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nIDEvent = <u32>::from_stack(mem, esp + 8u32);
            let uElapse = <u32>::from_stack(mem, esp + 12u32);
            let lpTimerFunc = <u32>::from_stack(mem, esp + 16u32);
            winapi::user32::SetTimer(machine, hWnd, nIDEvent, uElapse, lpTimerFunc).to_raw()
        }
        pub unsafe fn SetWindowPos(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hWndInsertAfter = <HWND>::from_stack(mem, esp + 8u32);
            let X = <i32>::from_stack(mem, esp + 12u32);
            let Y = <i32>::from_stack(mem, esp + 16u32);
            let cx = <i32>::from_stack(mem, esp + 20u32);
            let cy = <i32>::from_stack(mem, esp + 24u32);
            let uFlags = <Result<SWP, u32>>::from_stack(mem, esp + 28u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::SetWindowPos(
                        machine,
                        hWnd,
                        hWndInsertAfter,
                        X,
                        Y,
                        cx,
                        cy,
                        uFlags,
                    )
                    .await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 28u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::SetWindowPos(
                    machine,
                    hWnd,
                    hWndInsertAfter,
                    X,
                    Y,
                    cx,
                    cy,
                    uFlags
                ));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn SetWindowTextA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpString = <Option<&str>>::from_stack(mem, esp + 8u32);
            winapi::user32::SetWindowTextA(machine, hWnd, lpString).to_raw()
        }
        pub unsafe fn ShowCursor(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let bShow = <bool>::from_stack(mem, esp + 4u32);
            winapi::user32::ShowCursor(machine, bShow).to_raw()
        }
        pub unsafe fn ShowWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let nCmdShow = <Result<SW, u32>>::from_stack(mem, esp + 8u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::ShowWindow(machine, hWnd, nCmdShow).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 8u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::ShowWindow(machine, hWnd, nCmdShow));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn TranslateAcceleratorW(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let hAccTable = <u32>::from_stack(mem, esp + 8u32);
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 12u32);
            winapi::user32::TranslateAcceleratorW(machine, hWnd, hAccTable, lpMsg).to_raw()
        }
        pub unsafe fn TranslateMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpMsg = <Option<&MSG>>::from_stack(mem, esp + 4u32);
            winapi::user32::TranslateMessage(machine, lpMsg).to_raw()
        }
        pub unsafe fn UpdateWindow(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            #[cfg(feature = "x86-emu")]
            {
                let m: *mut Machine = machine;
                let result = async move {
                    use memory::Extensions;
                    let machine = unsafe { &mut *m };
                    let result = winapi::user32::UpdateWindow(machine, hWnd).await;
                    let cpu = &mut machine.emu.x86.cpu_mut();
                    cpu.regs.eip = x86::ops::pop(cpu, machine.emu.memory.mem());
                    *cpu.regs.get32_mut(x86::Register::ESP) += 4u32;
                    cpu.regs.set32(x86::Register::EAX, result.to_raw());
                };
                machine
                    .emu
                    .x86
                    .cpu_mut()
                    .call_async(machine.emu.memory.mem(), Box::pin(result));
                0
            }
            #[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
            {
                let pin = std::pin::pin!(winapi::user32::UpdateWindow(machine, hWnd));
                crate::shims::call_sync(pin).to_raw()
            }
        }
        pub unsafe fn ValidateRect(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hWnd = <HWND>::from_stack(mem, esp + 4u32);
            let lpRect = <Option<&RECT>>::from_stack(mem, esp + 8u32);
            winapi::user32::ValidateRect(machine, hWnd, lpRect).to_raw()
        }
        pub unsafe fn WaitMessage(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::user32::WaitMessage(machine).to_raw()
        }
        pub unsafe fn wsprintfA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let buf = <u32>::from_stack(mem, esp + 4u32);
            let fmt = <Option<&str>>::from_stack(mem, esp + 8u32);
            let args = <VarArgs>::from_stack(mem, esp + 12u32);
            winapi::user32::wsprintfA(machine, buf, fmt, args).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const AdjustWindowRect: Shim = Shim {
            name: "AdjustWindowRect",
            func: impls::AdjustWindowRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const AdjustWindowRectEx: Shim = Shim {
            name: "AdjustWindowRectEx",
            func: impls::AdjustWindowRectEx,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const AppendMenuA: Shim = Shim {
            name: "AppendMenuA",
            func: impls::AppendMenuA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const BeginPaint: Shim = Shim {
            name: "BeginPaint",
            func: impls::BeginPaint,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CheckMenuItem: Shim = Shim {
            name: "CheckMenuItem",
            func: impls::CheckMenuItem,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const ClientToScreen: Shim = Shim {
            name: "ClientToScreen",
            func: impls::ClientToScreen,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const CreateCursor: Shim = Shim {
            name: "CreateCursor",
            func: impls::CreateCursor,
            stack_consumed: 28u32,
            is_async: false,
        };
        pub const CreateWindowExA: Shim = Shim {
            name: "CreateWindowExA",
            func: impls::CreateWindowExA,
            stack_consumed: 48u32,
            is_async: true,
        };
        pub const CreateWindowExW: Shim = Shim {
            name: "CreateWindowExW",
            func: impls::CreateWindowExW,
            stack_consumed: 48u32,
            is_async: true,
        };
        pub const DefWindowProcA: Shim = Shim {
            name: "DefWindowProcA",
            func: impls::DefWindowProcA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const DefWindowProcW: Shim = Shim {
            name: "DefWindowProcW",
            func: impls::DefWindowProcW,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const DestroyWindow: Shim = Shim {
            name: "DestroyWindow",
            func: impls::DestroyWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const DialogBoxIndirectParamA: Shim = Shim {
            name: "DialogBoxIndirectParamA",
            func: impls::DialogBoxIndirectParamA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const DialogBoxParamA: Shim = Shim {
            name: "DialogBoxParamA",
            func: impls::DialogBoxParamA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const DispatchMessageA: Shim = Shim {
            name: "DispatchMessageA",
            func: impls::DispatchMessageA,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const DispatchMessageW: Shim = Shim {
            name: "DispatchMessageW",
            func: impls::DispatchMessageW,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const DrawTextW: Shim = Shim {
            name: "DrawTextW",
            func: impls::DrawTextW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const EndPaint: Shim = Shim {
            name: "EndPaint",
            func: impls::EndPaint,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FillRect: Shim = Shim {
            name: "FillRect",
            func: impls::FillRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const FindWindowA: Shim = Shim {
            name: "FindWindowA",
            func: impls::FindWindowA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const FrameRect: Shim = Shim {
            name: "FrameRect",
            func: impls::FrameRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const GetActiveWindow: Shim = Shim {
            name: "GetActiveWindow",
            func: impls::GetActiveWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetClientRect: Shim = Shim {
            name: "GetClientRect",
            func: impls::GetClientRect,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetDC: Shim = Shim {
            name: "GetDC",
            func: impls::GetDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetDesktopWindow: Shim = Shim {
            name: "GetDesktopWindow",
            func: impls::GetDesktopWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetFocus: Shim = Shim {
            name: "GetFocus",
            func: impls::GetFocus,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetForegroundWindow: Shim = Shim {
            name: "GetForegroundWindow",
            func: impls::GetForegroundWindow,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetKeyState: Shim = Shim {
            name: "GetKeyState",
            func: impls::GetKeyState,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetLastActivePopup: Shim = Shim {
            name: "GetLastActivePopup",
            func: impls::GetLastActivePopup,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const GetMessageA: Shim = Shim {
            name: "GetMessageA",
            func: impls::GetMessageA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const GetMessageW: Shim = Shim {
            name: "GetMessageW",
            func: impls::GetMessageW,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const GetSystemMenu: Shim = Shim {
            name: "GetSystemMenu",
            func: impls::GetSystemMenu,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetSystemMetrics: Shim = Shim {
            name: "GetSystemMetrics",
            func: impls::GetSystemMetrics,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetWindowDC: Shim = Shim {
            name: "GetWindowDC",
            func: impls::GetWindowDC,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const GetWindowLongA: Shim = Shim {
            name: "GetWindowLongA",
            func: impls::GetWindowLongA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const GetWindowRect: Shim = Shim {
            name: "GetWindowRect",
            func: impls::GetWindowRect,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const IntersectRect: Shim = Shim {
            name: "IntersectRect",
            func: impls::IntersectRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InvalidateRect: Shim = Shim {
            name: "InvalidateRect",
            func: impls::InvalidateRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const InvalidateRgn: Shim = Shim {
            name: "InvalidateRgn",
            func: impls::InvalidateRgn,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const IsIconic: Shim = Shim {
            name: "IsIconic",
            func: impls::IsIconic,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const IsRectEmpty: Shim = Shim {
            name: "IsRectEmpty",
            func: impls::IsRectEmpty,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const LoadAcceleratorsW: Shim = Shim {
            name: "LoadAcceleratorsW",
            func: impls::LoadAcceleratorsW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadBitmapA: Shim = Shim {
            name: "LoadBitmapA",
            func: impls::LoadBitmapA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadCursorA: Shim = Shim {
            name: "LoadCursorA",
            func: impls::LoadCursorA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadCursorW: Shim = Shim {
            name: "LoadCursorW",
            func: impls::LoadCursorW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadIconA: Shim = Shim {
            name: "LoadIconA",
            func: impls::LoadIconA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadIconW: Shim = Shim {
            name: "LoadIconW",
            func: impls::LoadIconW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadImageA: Shim = Shim {
            name: "LoadImageA",
            func: impls::LoadImageA,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const LoadImageW: Shim = Shim {
            name: "LoadImageW",
            func: impls::LoadImageW,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const LoadMenuA: Shim = Shim {
            name: "LoadMenuA",
            func: impls::LoadMenuA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadMenuW: Shim = Shim {
            name: "LoadMenuW",
            func: impls::LoadMenuW,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const LoadStringA: Shim = Shim {
            name: "LoadStringA",
            func: impls::LoadStringA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const LoadStringW: Shim = Shim {
            name: "LoadStringW",
            func: impls::LoadStringW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MapWindowPoints: Shim = Shim {
            name: "MapWindowPoints",
            func: impls::MapWindowPoints,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MessageBoxA: Shim = Shim {
            name: "MessageBoxA",
            func: impls::MessageBoxA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MessageBoxW: Shim = Shim {
            name: "MessageBoxW",
            func: impls::MessageBoxW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const MoveWindow: Shim = Shim {
            name: "MoveWindow",
            func: impls::MoveWindow,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const MsgWaitForMultipleObjects: Shim = Shim {
            name: "MsgWaitForMultipleObjects",
            func: impls::MsgWaitForMultipleObjects,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PeekMessageA: Shim = Shim {
            name: "PeekMessageA",
            func: impls::PeekMessageA,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PeekMessageW: Shim = Shim {
            name: "PeekMessageW",
            func: impls::PeekMessageW,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const PostMessageW: Shim = Shim {
            name: "PostMessageW",
            func: impls::PostMessageW,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const PostQuitMessage: Shim = Shim {
            name: "PostQuitMessage",
            func: impls::PostQuitMessage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const PtInRect: Shim = Shim {
            name: "PtInRect",
            func: impls::PtInRect,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const RegisterClassA: Shim = Shim {
            name: "RegisterClassA",
            func: impls::RegisterClassA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterClassExA: Shim = Shim {
            name: "RegisterClassExA",
            func: impls::RegisterClassExA,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterClassExW: Shim = Shim {
            name: "RegisterClassExW",
            func: impls::RegisterClassExW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterClassW: Shim = Shim {
            name: "RegisterClassW",
            func: impls::RegisterClassW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const RegisterWindowMessageW: Shim = Shim {
            name: "RegisterWindowMessageW",
            func: impls::RegisterWindowMessageW,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ReleaseCapture: Shim = Shim {
            name: "ReleaseCapture",
            func: impls::ReleaseCapture,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const ReleaseDC: Shim = Shim {
            name: "ReleaseDC",
            func: impls::ReleaseDC,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SendMessageA: Shim = Shim {
            name: "SendMessageA",
            func: impls::SendMessageA,
            stack_consumed: 16u32,
            is_async: true,
        };
        pub const SetCapture: Shim = Shim {
            name: "SetCapture",
            func: impls::SetCapture,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetCursor: Shim = Shim {
            name: "SetCursor",
            func: impls::SetCursor,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetFocus: Shim = Shim {
            name: "SetFocus",
            func: impls::SetFocus,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetForegroundWindow: Shim = Shim {
            name: "SetForegroundWindow",
            func: impls::SetForegroundWindow,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetMenu: Shim = Shim {
            name: "SetMenu",
            func: impls::SetMenu,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const SetMenuItemInfoA: Shim = Shim {
            name: "SetMenuItemInfoA",
            func: impls::SetMenuItemInfoA,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetRect: Shim = Shim {
            name: "SetRect",
            func: impls::SetRect,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const SetRectEmpty: Shim = Shim {
            name: "SetRectEmpty",
            func: impls::SetRectEmpty,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const SetTimer: Shim = Shim {
            name: "SetTimer",
            func: impls::SetTimer,
            stack_consumed: 16u32,
            is_async: false,
        };
        pub const SetWindowPos: Shim = Shim {
            name: "SetWindowPos",
            func: impls::SetWindowPos,
            stack_consumed: 28u32,
            is_async: true,
        };
        pub const SetWindowTextA: Shim = Shim {
            name: "SetWindowTextA",
            func: impls::SetWindowTextA,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const ShowCursor: Shim = Shim {
            name: "ShowCursor",
            func: impls::ShowCursor,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const ShowWindow: Shim = Shim {
            name: "ShowWindow",
            func: impls::ShowWindow,
            stack_consumed: 8u32,
            is_async: true,
        };
        pub const TranslateAcceleratorW: Shim = Shim {
            name: "TranslateAcceleratorW",
            func: impls::TranslateAcceleratorW,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const TranslateMessage: Shim = Shim {
            name: "TranslateMessage",
            func: impls::TranslateMessage,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const UpdateWindow: Shim = Shim {
            name: "UpdateWindow",
            func: impls::UpdateWindow,
            stack_consumed: 4u32,
            is_async: true,
        };
        pub const ValidateRect: Shim = Shim {
            name: "ValidateRect",
            func: impls::ValidateRect,
            stack_consumed: 8u32,
            is_async: false,
        };
        pub const WaitMessage: Shim = Shim {
            name: "WaitMessage",
            func: impls::WaitMessage,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const wsprintfA: Shim = Shim {
            name: "wsprintfA",
            func: impls::wsprintfA,
            stack_consumed: 0u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 90usize] = [
        shims::AdjustWindowRect,
        shims::AdjustWindowRectEx,
        shims::AppendMenuA,
        shims::BeginPaint,
        shims::CheckMenuItem,
        shims::ClientToScreen,
        shims::CreateCursor,
        shims::CreateWindowExA,
        shims::CreateWindowExW,
        shims::DefWindowProcA,
        shims::DefWindowProcW,
        shims::DestroyWindow,
        shims::DialogBoxIndirectParamA,
        shims::DialogBoxParamA,
        shims::DispatchMessageA,
        shims::DispatchMessageW,
        shims::DrawTextW,
        shims::EndPaint,
        shims::FillRect,
        shims::FindWindowA,
        shims::FrameRect,
        shims::GetActiveWindow,
        shims::GetClientRect,
        shims::GetDC,
        shims::GetDesktopWindow,
        shims::GetFocus,
        shims::GetForegroundWindow,
        shims::GetKeyState,
        shims::GetLastActivePopup,
        shims::GetMessageA,
        shims::GetMessageW,
        shims::GetSystemMenu,
        shims::GetSystemMetrics,
        shims::GetWindowDC,
        shims::GetWindowLongA,
        shims::GetWindowRect,
        shims::IntersectRect,
        shims::InvalidateRect,
        shims::InvalidateRgn,
        shims::IsIconic,
        shims::IsRectEmpty,
        shims::LoadAcceleratorsW,
        shims::LoadBitmapA,
        shims::LoadCursorA,
        shims::LoadCursorW,
        shims::LoadIconA,
        shims::LoadIconW,
        shims::LoadImageA,
        shims::LoadImageW,
        shims::LoadMenuA,
        shims::LoadMenuW,
        shims::LoadStringA,
        shims::LoadStringW,
        shims::MapWindowPoints,
        shims::MessageBoxA,
        shims::MessageBoxW,
        shims::MoveWindow,
        shims::MsgWaitForMultipleObjects,
        shims::PeekMessageA,
        shims::PeekMessageW,
        shims::PostMessageW,
        shims::PostQuitMessage,
        shims::PtInRect,
        shims::RegisterClassA,
        shims::RegisterClassExA,
        shims::RegisterClassExW,
        shims::RegisterClassW,
        shims::RegisterWindowMessageW,
        shims::ReleaseCapture,
        shims::ReleaseDC,
        shims::SendMessageA,
        shims::SetCapture,
        shims::SetCursor,
        shims::SetFocus,
        shims::SetForegroundWindow,
        shims::SetMenu,
        shims::SetMenuItemInfoA,
        shims::SetRect,
        shims::SetRectEmpty,
        shims::SetTimer,
        shims::SetWindowPos,
        shims::SetWindowTextA,
        shims::ShowCursor,
        shims::ShowWindow,
        shims::TranslateAcceleratorW,
        shims::TranslateMessage,
        shims::UpdateWindow,
        shims::ValidateRect,
        shims::WaitMessage,
        shims::wsprintfA,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/user32.dll"),
    };
}
pub mod wininet {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::wininet::*;
        pub unsafe fn InternetOpenA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let lpszAgent = <Option<&str>>::from_stack(mem, esp + 4u32);
            let dwAccessType = <u32>::from_stack(mem, esp + 8u32);
            let lpszProxy = <Option<&str>>::from_stack(mem, esp + 12u32);
            let lpszProxyBypass = <Option<&str>>::from_stack(mem, esp + 16u32);
            let dwFlags = <u32>::from_stack(mem, esp + 20u32);
            winapi::wininet::InternetOpenA(
                machine,
                lpszAgent,
                dwAccessType,
                lpszProxy,
                lpszProxyBypass,
                dwFlags,
            )
            .to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const InternetOpenA: Shim = Shim {
            name: "InternetOpenA",
            func: impls::InternetOpenA,
            stack_consumed: 20u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 1usize] = [shims::InternetOpenA];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "wininet.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/wininet.dll"),
    };
}
pub mod winmm {
    use super::*;
    mod impls {
        use crate::{
            machine::Machine,
            winapi::{self, stack_args::*, types::*},
        };
        use memory::Extensions;
        use winapi::winmm::*;
        pub unsafe fn timeBeginPeriod(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uPeriod = <u32>::from_stack(mem, esp + 4u32);
            winapi::winmm::timeBeginPeriod(machine, uPeriod).to_raw()
        }
        pub unsafe fn timeGetTime(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::timeGetTime(machine).to_raw()
        }
        pub unsafe fn timeSetEvent(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDelay = <u32>::from_stack(mem, esp + 4u32);
            let uResolution = <u32>::from_stack(mem, esp + 8u32);
            let lpTimeProc = <u32>::from_stack(mem, esp + 12u32);
            let dwUser = <u32>::from_stack(mem, esp + 16u32);
            let fuEvent = <u32>::from_stack(mem, esp + 20u32);
            winapi::winmm::timeSetEvent(machine, uDelay, uResolution, lpTimeProc, dwUser, fuEvent)
                .to_raw()
        }
        pub unsafe fn waveOutClose(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutClose(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutGetDevCapsA(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let uDeviceID = <u32>::from_stack(mem, esp + 4u32);
            let pwoc = <Option<&mut WAVEOUTCAPS>>::from_stack(mem, esp + 8u32);
            let cbwoc = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetDevCapsA(machine, uDeviceID, pwoc, cbwoc).to_raw()
        }
        pub unsafe fn waveOutGetNumDevs(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            winapi::winmm::waveOutGetNumDevs(machine).to_raw()
        }
        pub unsafe fn waveOutGetPosition(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pmmt = <Option<&mut MMTIME>>::from_stack(mem, esp + 8u32);
            let cbmmt = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutGetPosition(machine, hwo, pmmt, cbmmt).to_raw()
        }
        pub unsafe fn waveOutOpen(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let phwo = <Option<&mut HWAVEOUT>>::from_stack(mem, esp + 4u32);
            let uDeviceID = <u32>::from_stack(mem, esp + 8u32);
            let pwfx = <Option<&WAVEFORMATEX>>::from_stack(mem, esp + 12u32);
            let dwCallback = <u32>::from_stack(mem, esp + 16u32);
            let dwInstance = <u32>::from_stack(mem, esp + 20u32);
            let fdwOpen = <Result<WaveOutOpenFlags, u32>>::from_stack(mem, esp + 24u32);
            winapi::winmm::waveOutOpen(
                machine, phwo, uDeviceID, pwfx, dwCallback, dwInstance, fdwOpen,
            )
            .to_raw()
        }
        pub unsafe fn waveOutPrepareHeader(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutPrepareHeader(machine, hwo, pwh, cbwh).to_raw()
        }
        pub unsafe fn waveOutReset(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            winapi::winmm::waveOutReset(machine, hwo).to_raw()
        }
        pub unsafe fn waveOutWrite(machine: &mut Machine, esp: u32) -> u32 {
            let mem = machine.mem().detach();
            let hwo = <HWAVEOUT>::from_stack(mem, esp + 4u32);
            let pwh = <Option<&WAVEHDR>>::from_stack(mem, esp + 8u32);
            let cbwh = <u32>::from_stack(mem, esp + 12u32);
            winapi::winmm::waveOutWrite(machine, hwo, pwh, cbwh).to_raw()
        }
    }
    mod shims {
        use super::impls;
        use super::Shim;
        pub const timeBeginPeriod: Shim = Shim {
            name: "timeBeginPeriod",
            func: impls::timeBeginPeriod,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const timeGetTime: Shim = Shim {
            name: "timeGetTime",
            func: impls::timeGetTime,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const timeSetEvent: Shim = Shim {
            name: "timeSetEvent",
            func: impls::timeSetEvent,
            stack_consumed: 20u32,
            is_async: false,
        };
        pub const waveOutClose: Shim = Shim {
            name: "waveOutClose",
            func: impls::waveOutClose,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const waveOutGetDevCapsA: Shim = Shim {
            name: "waveOutGetDevCapsA",
            func: impls::waveOutGetDevCapsA,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutGetNumDevs: Shim = Shim {
            name: "waveOutGetNumDevs",
            func: impls::waveOutGetNumDevs,
            stack_consumed: 0u32,
            is_async: false,
        };
        pub const waveOutGetPosition: Shim = Shim {
            name: "waveOutGetPosition",
            func: impls::waveOutGetPosition,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutOpen: Shim = Shim {
            name: "waveOutOpen",
            func: impls::waveOutOpen,
            stack_consumed: 24u32,
            is_async: false,
        };
        pub const waveOutPrepareHeader: Shim = Shim {
            name: "waveOutPrepareHeader",
            func: impls::waveOutPrepareHeader,
            stack_consumed: 12u32,
            is_async: false,
        };
        pub const waveOutReset: Shim = Shim {
            name: "waveOutReset",
            func: impls::waveOutReset,
            stack_consumed: 4u32,
            is_async: false,
        };
        pub const waveOutWrite: Shim = Shim {
            name: "waveOutWrite",
            func: impls::waveOutWrite,
            stack_consumed: 12u32,
            is_async: false,
        };
    }
    const SHIMS: [Shim; 11usize] = [
        shims::timeBeginPeriod,
        shims::timeGetTime,
        shims::timeSetEvent,
        shims::waveOutClose,
        shims::waveOutGetDevCapsA,
        shims::waveOutGetNumDevs,
        shims::waveOutGetPosition,
        shims::waveOutOpen,
        shims::waveOutPrepareHeader,
        shims::waveOutReset,
        shims::waveOutWrite,
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        shims: &SHIMS,
        raw: std::include_bytes!("../../dll/winmm.dll"),
    };
}
