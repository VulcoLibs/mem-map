use super::*;


lazy_static! {
    pub static ref MEM_MAP: Mutex<HashMap<u64, MemWrapper>> = {
        Mutex::new(Default::default())
    };
}

#[repr(C)]
pub struct MemWrapper {
    pub ptr: *mut c_void
}

unsafe impl Send for MemWrapper {}
