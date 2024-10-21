#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::shlwapi::*;
    pub unsafe fn PathRemoveFileSpecA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let pszPath = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::trace::enabled("shlwapi") {
            crate::trace::Record::new(
                winapi::shlwapi::PathRemoveFileSpecA_pos,
                "shlwapi",
                "PathRemoveFileSpecA",
                &[("pszPath", &pszPath)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::shlwapi::PathRemoveFileSpecA(machine, pszPath);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "PathRemoveFileSpecA",
    func: Handler::Sync(wrappers::PathRemoveFileSpecA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "shlwapi.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/shlwapi.dll"),
};
