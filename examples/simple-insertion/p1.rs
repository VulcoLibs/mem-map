use mem_map::c_void;
use std::thread::sleep;
use std::time::Duration;

fn main() { unsafe {
    let mut def_val = 0_u32;
    let mut val = 0_u32;

    mem_map::init();

    mem_map::insert_cp(1, (&mut val as *mut u32) as *mut c_void, std::mem::size_of::<u32>(), true);

    println!("PID: {}\n---------------", std::process::id());

    loop {
        let x = *(mem_map::get(1) as *mut u32);

        if x != def_val {
            println!("Insertion successful!\nNew value: {}\n", x);
            def_val = x;
        }

        sleep(Duration::from_millis(50));
    }
} }
