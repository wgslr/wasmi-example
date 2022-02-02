// declare functions which we expect to be provided by the runtime
extern "C" {
    fn get_current_year() -> i32;
}

#[no_mangle] // no_mangle ensures the function name will not be modified
             // pub extern "C" means the function can be called from outside WASM
pub extern "C" fn is_it_leap_year_now() -> i32 {
    // calls to extern functions are always unsafe
    let current_year: i32 = unsafe { get_current_year() };
    return is_leap_year(current_year);
}

// no_mangle preserves the name `is_leap_year`
// pub extern "C" allows the function to be called from outside WASM
#[no_mangle]
pub extern "C" fn is_leap_year(year: i32) -> i32 {
    // Logically, this function returns a boolean value.
    // However, the binary WebAssembly  format supports only integer types.
    // To avoid inconsistency when we move to invoking this function from Rust,
    // I'm using i32 explicitly here.

    if year % 4 == 0 && (year % 100 != 0 || (year % 400 == 0)) {
        return 1;
    } else {
        return 0;
    }
}
