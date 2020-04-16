use ctor::ctor;
use nj_sys as sys;
use std::{ffi::CString, ptr};

macro_rules! fixup {
    ($name: expr, ($($t:ty),*)) => {{
        #[cfg(windows)]
        {
            let f: unsafe extern "C" fn($($t),*) -> _ = $name;
            let addr = lookup(f, stringify!($name));

            unsafe {
                println!("detouring {:?} => {:?}", $name as *const (), addr as *const ());
                let detour = detour::RawDetour::new(
                    $name as *const (),
                    addr as *const (),
                ).unwrap();
                detour.enable().unwrap();
                Box::leak(Box::new(detour));
            }
        }
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
    println!("looked up {:?} => {:?}", name, addr);
    unsafe { std::mem::transmute_copy(&addr) }
}

#[ctor]
#[no_mangle]
fn ctor() {
    println!("Hello from wallet");

    // TODO: fix up everything
    fixup!(sys::napi_module_register, (_));
    fixup!(sys::napi_create_object, (_, _));
    fixup!(sys::napi_create_string_utf8, (_, _, _, _));

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
        sys::napi_module_register(&mut module);
        println!("called it!");
    }
}

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
