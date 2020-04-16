use ctor::ctor;
use nj_sys as sys;
use std::{ffi::CString, ptr};

#[ctor]
#[no_mangle]
fn ctor() {
    println!("Hello from wallet");
    unsafe {
        let modname = CString::new("wallet").unwrap();
        let filename = CString::new("lib.rs").unwrap();
        let mut module = sys::napi_module {
            // see https://nodejs.org/api/n-api.html#n_api_n_api_version_matrix
            nm_version: sys::NAPI_VERSION as i32,
            nm_flags: 0,
            nm_filename: filename.as_ptr(),
            nm_modname: modname.as_ptr(),
            nm_register_func: Some(register),
            nm_priv: ptr::null_mut(),
            reserved: [
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            ],
        };
        sys::napi_module_register(&mut module);
    }
}

#[no_mangle]
unsafe extern "C" fn register(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    println!("In register! exports = {:?}", exports);

    let mut ret: sys::napi_value = ptr::null_mut();
    sys::napi_create_object(env, &mut ret);

    ret
}
