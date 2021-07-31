use super::*;


pub fn get_exe() -> String {
    format!("{}", std::env::current_exe().unwrap().file_name().unwrap().to_str().unwrap())
}

pub fn get_key_path() -> String {
    format!("{}\\{}", REG_PATH, get_exe())
}

pub fn save_address(id: &u64, addr: &String) -> bool {
    return match HKCU.create_subkey(get_key_path()) {
        Ok((key, _)) => {
            match key.set_value(format!("{}", id), addr) {
                Ok(_) => {
                    true
                }
                Err(error) => {
                    println!("{}", error);
                    false
                }
            }
        }

        Err(error) => {
            println!("{}", error);
            false
        }
    }
}

pub unsafe fn insert(id: u64, ptr: *mut c_void, size: size_t, as_extern: bool, copy: bool) -> u32 {
    if id == 0 {
        return Error::RestrictedID.into();
    }

    let mut map = MEM_MAP.lock().unwrap();
    map.insert(id, MemWrapper{ptr: malloc(size)});

    if copy {
        memcpy(
            map.get(&id).unwrap().ptr,
            ptr,
            size,
        );
    } else {
        memmove(
            map.get(&id).unwrap().ptr,
            ptr,
            size,
        );
    }

    if as_extern {
        if !save_address(
            &id,
            &format!("{:?}", map.get(&id).unwrap() as *const MemWrapper).replace("0x", ""),
        ) {
            return Error::AddressNotSaved.into();
        }
    }

    Error::None.into()
}
