use super::*;



/// [`init`] - Initializes the `mem-map` library.
///
/// Should be called only once at the start of a program.
///
/// Returns `false` if failed and `true` if succeeded.
///
#[no_mangle]
pub unsafe extern "C" fn init() -> bool {
    let exe = get_exe();

    match HKCU.create_subkey(REG_PATH) {
        Ok((key, _)) => {
            if let Err(_) = key.set_value(
                &exe,
                &format!("{}", std::process::id()),
            ) {
                return false;
            }
        },
        Err(_) => {
            return false;
        }
    }

    let key_path = get_key_path();

    let _ = HKCU.delete_subkey(&key_path);

    return match HKCU.create_subkey(&key_path) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}


/// [`insert_cp`] - Copies a certain memory region into an own map
///
/// Uses `memcpy` to copy the memory
///
/// Returns non zero value if failed. <br>
/// See the [error enum](Error) to find out more.
///
/// # Arguments
/// * `id: u64` - Map key, used to later get the pointer.
/// * `ptr: *mut c_void` - Pointer to the data to copy.
/// * `size: size_t` - Size of the data to copy.
/// * `as_extern: bool` - Should the data be available for other processes.
///
#[no_mangle]
pub unsafe extern "C" fn insert_cp(id: u64, ptr: *mut c_void, size: size_t, as_extern: bool) -> u32 {
    insert(id, ptr, size, as_extern, true)
}


/// [`insert_mv`] - Copies a certain memory region into an own map
///
/// Uses `memmove` to copy the memory
///
/// Returns non zero value if failed. <br>
/// See the [error enum](Error) to find out more.
///
/// # Arguments
/// * `id: u64` - Map key, used to later get the pointer.
/// * `ptr: *mut c_void` - Pointer to the data to copy.
/// * `size: size_t` - Size of the data to copy.
/// * `as_extern: bool` - Should the data be available for other processes.
///
#[no_mangle]
pub unsafe extern "C" fn insert_mv(id: u64, ptr: *mut c_void, size: size_t, as_extern: bool) -> u32 {
    insert(id, ptr, size, as_extern, false)
}


/// [`get`] - Gets a pointer to saved data
///
/// Returns `null` pointer when:
/// - restricted id was provided
/// - no entry in map was found at provided id
///
/// # Arguments
/// * `id: u64` - Map key.
///
#[no_mangle]
pub unsafe extern "C" fn get(id: u64) -> *mut c_void {
    if id == 0 {
        return NULL;
    }

    let map = MEM_MAP.lock().unwrap();

    if !map.contains_key(&id) {
        return NULL;
    }

    map.get(&id).unwrap().ptr
}


/// [`get_from_reg`] - Gets a pointer to saved data by registry entry.
///
/// Returns `null` pointer when:
/// - failed to find the registry entry.
///
/// # Arguments
/// * `id: u64` - Map key.
///
#[deprecated]
#[no_mangle]
pub unsafe extern "C" fn get_from_reg(id: u64) -> *mut c_void {
    let addr = match HKCU.open_subkey(format!("{}\\{}", REG_PATH, get_exe())) {
        Ok(key) => {
            if let Ok(addr) = key.get_value::<String, String>(format!("{}", id)) {
                addr
            } else {
                String::new()
            }
        }
        Err(_) => {
            String::new()
        }
    };

    if addr.is_empty() {
        return NULL;
    }

    let address = u64::from_str_radix(&addr, 16).unwrap();
    let ptr = address as *const MemWrapper;

    (*ptr).ptr
}

/// [`remove`] - Removes the saved data
///
/// Returns non zero value if failed. <br>
/// See the [error enum](Error) to find out more.
///
/// # Arguments
/// * `id: u64` - Map key.
///
#[no_mangle]
pub unsafe extern "C" fn remove(id: u64) -> u32 {
    if id == 0 {
        return Error::RestrictedID.into();
    }

    let mut map = MEM_MAP.lock().unwrap();

    if !map.contains_key(&id) {
        return Error::InvalidID.into();
    }

    let mem_wrapper_ptr = map.get(&id).unwrap().ptr;

    if !mem_wrapper_ptr.is_null() {
        dealloc(mem_wrapper_ptr as *mut _, Layout::for_value_raw(mem_wrapper_ptr as *const _));
    }

    map.remove(&id);

    match HKCU.create_subkey(format!("{}\\{}", REG_PATH, get_exe())) {
        Ok((key, _)) => {
            let _ = key.delete_value(format!("{}", id));
        }
        _ => {}
    }

    Error::None.into()
}


/// [`get_extern`] - Saves a data from another process into buffer.
///
/// Returns non zero value if failed. <br>
/// See the [error enum](Error) to find out more.
///
/// # Arguments
/// * `id: u64` - Map key.
/// * `pid: u32` - Target process' ID.
/// * `target: *mut c_void` - Pointer to a buffer that receives the data.
/// * `target_size: size_t` - Size of the buffer.
///
#[no_mangle]
pub unsafe extern "C" fn get_extern(id: u64, pid: u32, target: *mut c_void, target_size: size_t) -> u32 {
    if id == 0 {
        return Error::RestrictedID.into();
    }

    let handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
        FALSE,
        pid
    );

    if handle.is_null() {
        return Error::InvalidHandle.into();
    }

    let clear = |handle: *mut c_void, allocated: &[*mut c_void]| {
        if !handle.is_null() {
            CloseHandle(handle as *mut _);
        }

        for ptr in allocated {
            if !ptr.is_null() {
                dealloc((*ptr) as *mut _, Layout::for_value_raw(*ptr as *const _))
            }
        }
    };

    let mut name_buff = [0; 2048];

    GetModuleFileNameExW(
        handle,
        NULL as *mut _,
        name_buff.as_mut_ptr(),
        (mem::size_of_val(&name_buff) - 1) as u32,
    );

    let name = String::from_utf16(&name_buff).unwrap();
    let exe = std::path::Path::new(&name).file_name().unwrap().to_str().unwrap();

    let addr = match HKCU.open_subkey(format!("{}\\{}", REG_PATH, &exe)) {
        Ok(key) => {
            if let Ok(addr) = key.get_value::<String, String>(format!("{}", id)) {
                addr
            } else {
                String::new()
            }
        }
        Err(_) => {
            String::new()
        }
    };

    if addr.is_empty() {
        clear(handle as *mut _, &[NULL]);
        return Error::InvalidID.into();
    }

    let address = u64::from_str_radix(&addr, 16).unwrap();
    let ptr = address as *mut MemWrapper;
    let mut size = std::mem::size_of::<MemWrapper>();
    let buffer = libc::malloc(size) as *mut MemWrapper;
    let bytes_saved = libc::malloc(std::mem::size_of::<usize>()) as *mut usize;

    ReadProcessMemory(
        handle,
        ptr as *mut _,
        buffer as *mut _,
        size,
        bytes_saved,
    );

    if bytes_saved.is_null() || buffer.is_null() {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    if (*bytes_saved) == 0 {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    size = target_size;
    libc::realloc(buffer as *mut c_void, std::mem::size_of::<*mut c_void>());
    libc::realloc(bytes_saved as *mut c_void, std::mem::size_of::<usize>());

    ReadProcessMemory(
        handle,
        (*buffer).ptr as *mut _,
        buffer as *mut _,
        size,
        bytes_saved,
    );

    if (*bytes_saved) == 0 {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    if buffer.is_null() {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    memmove(target as *mut c_void, buffer as *mut c_void, size);
    clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
    Error::None.into()
}


/// [`insert_extern`] - Copies a certain memory region into a map of another process.
///
/// Returns non zero value if failed. <br>
/// See the [error enum](Error) to find out more.
///
/// # Arguments
/// * `id: u64` - Map key.
/// * `pid: u32` - Target process' ID.
/// * `source: *mut c_void` - Pointer to a buffer that the data is read from.
/// * `source_size: size_t` - Size of the buffer.
///
#[no_mangle]
pub unsafe extern "C" fn insert_extern(id: u64, pid: u32, source: *const c_void, source_size: size_t) -> u32 {
    if id == 0 {
        return Error::RestrictedID.into();
    }

    let handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
        FALSE,
        pid
    );

    if handle.is_null() {
        return Error::InvalidHandle.into();
    }

    let clear = |handle: *mut c_void, allocated: &[*mut c_void]| {
        if !handle.is_null() {
            CloseHandle(handle as *mut _);
        }

        for memory in allocated {
            if !memory.is_null() {
                dealloc((*memory) as *mut _, Layout::for_value_raw(*memory as *const _))
            }
        }
    };

    let mut name_buff = [0; 2048];

    GetModuleFileNameExW(
        handle,
        NULL as *mut _,
        name_buff.as_mut_ptr(),
        (mem::size_of_val(&name_buff) - 1) as u32,
    );

    let name = String::from_utf16(&name_buff).unwrap();
    let exe = std::path::Path::new(&name).file_name().unwrap().to_str().unwrap();

    let addr = match HKCU.open_subkey(format!("{}\\{}", REG_PATH, &exe)) {
        Ok(key) => {
            if let Ok(addr) = key.get_value::<String, String>(format!("{}", id)) {
                addr
            } else {
                String::new()
            }
        }
        Err(_) => {
            String::new()
        }
    };

    if addr.is_empty() {
        clear(handle as *mut _, &[NULL]);
        return Error::InvalidID.into();
    }

    let address = u64::from_str_radix(&addr, 16).unwrap();
    let ptr = address as *mut MemWrapper;
    let s = std::mem::size_of::<MemWrapper>();
    let buffer = libc::malloc(source_size) as *mut MemWrapper;
    let bytes_saved = libc::malloc(std::mem::size_of::<usize>()) as *mut usize;

    ReadProcessMemory(
        handle,
        ptr as *mut _,
        buffer as *mut _,
        s,
        bytes_saved,
    );

    if bytes_saved.is_null() || buffer.is_null() {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    if (*bytes_saved) == 0 {
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    libc::realloc(bytes_saved as *mut _, std::mem::size_of::<usize>());

    WriteProcessMemory(
        handle,
        (*buffer).ptr as *mut _,
        source as *const _,
        source_size,
        bytes_saved,
    );

    if (*bytes_saved) == 0 {
        println!("FAIL");
        clear(handle as *mut _, &[buffer as *mut _, bytes_saved as *mut _]);
        return Error::ReadProcessMemoryFailed.into();
    }

    Error::None.into()
}


/// [`id_exists`] - Checks if certain map key (ID) exists.
///
/// Returns `true` if ID exists and `false` if not
///
/// # Arguments
/// * `id: u64` - Map key.
/// * `pid: u32` - Target process' ID.
#[no_mangle]
pub unsafe extern "C" fn id_exists(id: u64, pid: u32) -> bool {
    let mut key_name = String::new();

    if match HKCU.create_subkey(REG_PATH) {
        Ok((key, _disposition)) => {
            let pid_str = format!("{}", pid);

            key.enum_values().any(|x| {
                if x.is_ok() {
                    if let Ok(value_pid) = String::from_reg_value(&(x.as_ref().unwrap().1)) {
                        if value_pid == pid_str {
                            drop(mem::replace(&mut key_name, x.unwrap().0));
                            true
                        } else {false}
                    } else {false}
                } else {false}
            })
        }
        Err(_) => false,
    } == false {
        return false;
    }

    match HKCU.open_subkey(format!("{}\\{}", REG_PATH, key_name)) {
        Ok(key) => {
            key.get_value::<String, String>(format!("{}", id)).is_ok()
        }
        Err(_error) => {
            false
        }
    }
}
