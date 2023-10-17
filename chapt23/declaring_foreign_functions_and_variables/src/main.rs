use std::ffi::CStr;
use std::os::raw::c_char;

extern {
    fn strlen(s: *const c_char) -> usize;
}

extern {
    static environ: *mut *mut c_char;
}


fn main() {
    println!("Declaring Foreign Functions and Variables test!");
}

#[test]
fn test_extern() {
    use std::ffi::CString;

    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();
    unsafe {
        assert_eq!(strlen(null_terminated.as_ptr()), 12);
    }
}

#[test]
fn test_global_var() {
    unsafe {
        if !environ.is_null() && !(*environ).is_null() {
            let var = CStr::from_ptr(*environ);
            println!("first environment variable: {}",
                var.to_string_lossy())
        }
    }

}
