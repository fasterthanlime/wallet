use nj_sys as sys;
use std::{ffi::CString, ptr};

#[cfg(windows)]
mod winhook;

macro_rules! fixup {
    ($name: expr, ($($t:ty),*)) => {{
        #[cfg(windows)]
        unsafe {
            let thunk: unsafe extern "C" fn($($t),*) -> _ = $name;
            winhook::hook(stringify!($name), thunk as *const std::ffi::c_void);
        }
    }};
}

#[no_mangle]
fn ctor() {
    println!("Hello from wallet");

    // TODO: fix up everything
    fixup!(sys::napi_module_register, (_));
    fixup!(sys::napi_create_object, (_, _));
    fixup!(sys::napi_create_string_utf8, (_, _, _, _));

    // union ungodly {
    //     c: unsafe extern "C" fn(*mut sys::napi_module),
    //     s: unsafe extern "stdcall" fn(*mut sys::napi_module),
    // };
    // let u = ungodly {
    //     c: sys::napi_module_register,
    // };
    // let napi_module_register = unsafe { u.s };

    unsafe {
        let modname = CString::new("wallet").unwrap();
        let filename = CString::new("lib.rs").unwrap();
        let mut module = sys::napi_module {
            // see https://nodejs.org/api/n-api.html#n_api_n_api_version_matrix
            nm_version: sys::NAPI_VERSION as i32,
            nm_flags: 0,
            nm_filename: filename.as_ptr(),
            nm_modname: modname.as_ptr(),
            nm_register_func: Some(init),
            nm_priv: ptr::null_mut(),
            reserved: [
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            ],
        };
        let module = Box::leak(Box::new(module));

        println!("calling it...");
        sys::napi_module_register(module);
        println!("called it!");

        debug_hook();
    }
}

#[no_mangle]
#[inline(never)]
fn debug_hook() {}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    println!("In init! exports = {:?}", exports);

    let mut ret: sys::napi_value = ptr::null_mut();
    sys::napi_create_object(env, &mut ret);

    let mut s = ptr::null_mut();
    let s_src = "Just yanking yer chain";
    sys::napi_create_string_utf8(env, s_src.as_ptr() as *const i8, s_src.len(), &mut s);

    s
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
pub static CTOR_ENTRY: extern "C" fn() = {
    extern "C" fn ctor_thunk() {
        ctor();
    };
    ctor_thunk
};
