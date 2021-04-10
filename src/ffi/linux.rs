use std::{
    ffi::{c_void, CStr},
    os::raw::c_char,
    ptr,
};

extern "C" {
    pub fn free(ptr: *mut c_void);
}

#[link(name = "dl")]
extern "C" {
    fn dlclose(handle: *mut c_void) -> i32;
    fn dlopen(file_name: *const c_char, flags: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub struct Library {
    lib: *mut c_void,
}

impl Library {
    pub fn loadc(&self, symbol: &CStr) -> *mut c_void {
        unsafe { dlsym(self.lib, symbol.as_ptr()) }
    }

    pub fn load(&self, symbol: &str) -> *mut c_void {
        let nul_name: Box<[u8]> = symbol.bytes().chain(std::iter::once(0)).collect();
        let csymbol = match CStr::from_bytes_with_nul(&nul_name).ok() {
            Some(s) => s,
            None => return ptr::null_mut(),
        };

        return unsafe { dlsym(self.lib, csymbol.as_ptr()) };
    }

    pub fn open(name: &str) -> Option<Self> {
        let prefix = "lib";
        let suffix = ".so";
        let full_name = prefix.bytes().chain(name.bytes()).chain(suffix.bytes());

        let terminated_name: Box<[u8]> = full_name.chain(std::iter::once(0)).collect();
        let cname = CStr::from_bytes_with_nul(&terminated_name).ok()?;

        let rtld_lazy = 0x00001;
        let lib = unsafe { dlopen(cname.as_ptr(), rtld_lazy) };
        if lib == ptr::null_mut() {
            return None;
        }

        return Some(Self { lib });
    }

    pub fn openc(name: &CStr) -> Option<Self> {
        let rtld_lazy = 0x00001;
        let lib = unsafe { dlopen(name.as_ptr(), rtld_lazy) };
        if lib == ptr::null_mut() {
            return None;
        }

        return Some(Self { lib });
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.lib);
        }
    }
}
