use nj_sys as sys;
use std::{ffi::CString, ptr};

macro_rules! fixup {
    ($name: expr, ($($t:ty),*)) => {{
        #[cfg(windows)]
        unsafe {
            const PAGE_EXECUTE_READWRITE: u32 = 0x40;
            use winapi::ctypes::c_void;

            let f: unsafe extern "C" fn($($t),*) -> _ = $name;
            let real = lookup(f, stringify!($name));

            println!("name = {:?}, real = {:?}", $name as *const (), real as *const ());
            let diff: isize = $name as isize - real as isize;
            println!("diff = {:?}, i32::max = {:?}", diff, std::i32::MAX);

            // TODO: support 32-bit
            let mut template: Vec<u8> = vec![0x48, 0xb8];
            template.extend(&(real as usize).to_le_bytes());
            template.extend(&[0xff, 0xe0]);
            print!("full template is {} bytes:", template.len());
            for b in &template {
                print!(" {:02x}", b);
            }
            println!();

            let mut old_prot = 0;
            let new_prot = PAGE_EXECUTE_READWRITE;
            println!("de-protecting...");
            let ret = winapi::um::memoryapi::VirtualProtect(
                $name as *mut c_void,
                template.len(),
                new_prot,
                &mut old_prot,
            );
            println!("ret = {:?}", ret);
            if (ret == 0) {
                panic!("could not de-protect");
            }

            println!("copying...");
            std::ptr::copy_nonoverlapping(
                template.as_ptr(),
                $name as *mut u8,
                template.len(),
            );

            println!("re-protecting...");
            let ret = winapi::um::memoryapi::VirtualProtect(
                $name as *mut c_void,
                template.len(),
                old_prot,
                &mut old_prot,
            );
            println!("ret = {:?}", ret);
            if (ret == 0) {
                panic!("could not re-protect");
            }

            // println!("detouring {:?} => {:?}", $name as *const (), real as *const ());
            // let detour = detour::RawDetour::new(
            //     $name as *const (),
            //     real as *const (),
            // ).unwrap();
            // detour.enable().unwrap();
            // Box::leak(Box::new(detour));
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
