use ctor::ctor;
use nj_sys as sys;
use std::{ffi::CString, ptr};

macro_rules! lookup {
    ($name: expr, ($($t:ty),*)) => {{
        let _f: unsafe extern "C" fn($($t),*) -> _ = $name;
        lookup(_f, stringify!($name))
    }};
}

#[cfg(windows)]
fn lookup<T>(_t: T, name: &str) -> T {
    let name = name.split("::").last().unwrap();
    let name = CString::new(name).unwrap();
    let addr = unsafe { winapi::um::libloaderapi::GetProcAddress(ptr::null_mut(), name.as_ptr()) };
    if addr.is_null() {
        panic!("could not find {:?}", name);
    }
    println!("looked up {:?} to {:?}", name, addr);
    unsafe { std::mem::transmute_copy(&addr) }
}

#[cfg(not(windows))]
fn lookup<T>(t: T, name: &str) -> T {
    t
}

#[ctor]
#[no_mangle]
fn ctor() {
    println!("Hello from wallet");

    let napi_module_register = lookup!(sys::napi_module_register, (_));

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
        println!("calling it...");
        napi_module_register(&mut module);
        println!("called it!");
    }
}

#[no_mangle]
unsafe extern "C" fn init(env: sys::napi_env, exports: sys::napi_value) -> sys::napi_value {
    println!("In init! exports = {:?}", exports);

    let napi_create_object = lookup!(sys::napi_create_object, (_, _));
    let napi_create_string_utf8 = lookup!(sys::napi_create_string_utf8, (_, _, _, _));

    let mut ret: sys::napi_value = ptr::null_mut();
    napi_create_object(env, &mut ret);

    let mut s = ptr::null_mut();
    let s_src = "Just yanking yer chain";
    napi_create_string_utf8(env, s_src.as_ptr() as *const i8, s_src.len(), &mut s);

    // ret
    s
}
