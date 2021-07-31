use mem_map::c_void;


/// Copied from: https://users.rust-lang.org/t/how-to-get-user-input/5176/3
fn input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}


fn main() { unsafe {
    print!("PID: ");
    let pid = u32::from_str_radix(input().as_str(), 10).unwrap();
    let mut value_extern = 0_u32;
    println!("---------------");

    loop {
        print!("u32 value to insert: ");
        let value = match u32::from_str_radix(input().as_str(), 10) {
            Ok(value) => value,
            Err(_) => {
                println!("Not a u32 value!");
                continue;
            }
        };

        mem_map::insert_extern(1, pid, (&value as *const u32) as *const _, std::mem::size_of::<u32>());

        if mem_map::get_extern(1, pid, (&mut value_extern as *mut u32) as *mut _, std::mem::size_of::<u32>()) == 0 {
            print!("extern value: {}", value_extern);
        };

        println!();
    }
} }
