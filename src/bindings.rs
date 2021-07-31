pub use std::collections::HashMap;
pub use std::sync::Mutex;
pub use std::mem;
pub use std::alloc::{
    Layout,
    dealloc,
};

pub use winreg::enums::*;
pub use winreg::types::FromRegValue;
pub use winreg::{
    RegKey,
    RegValue,
};

pub use libc::{
    size_t,
    c_void,
    malloc,
    memcpy,
    memmove,
};

pub use winapi::shared::minwindef::FALSE;
pub use winapi::um::processthreadsapi::OpenProcess;
pub use winapi::um::handleapi::CloseHandle;
pub use winapi::um::winnt::{
    PROCESS_VM_READ,
    PROCESS_VM_WRITE,
    PROCESS_QUERY_INFORMATION,
    PROCESS_VM_OPERATION
};

pub use winapi::um::psapi::{
    GetModuleFileNameExA,
    GetModuleFileNameExW,
};
pub use winapi::um::memoryapi::{
    ReadProcessMemory,
    WriteProcessMemory
};
