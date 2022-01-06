use libc::{c_int, c_char};
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::NonNull;

type FooCallback = unsafe extern "C" fn(msg: *const c_char);

mod get;
use get::http_get;
mod sleep;

use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn hget(url: Option<NonNull<c_char>>, cb: Option<FooCallback>, wait: c_int) {
    let wait: bool = wait != 0;
    if let None = cb {
        log::error!("hget: cb is null");
        return;
    }
    let cb = cb.unwrap();
    if let None = url {
        log::error!("hget: url is null");
        return;
    }
    let url = url.unwrap().as_ptr();
    log::debug!("hget: url at {:?}", url);
    let url = unsafe { CStr::from_ptr(url) };
    let url = CString::from(url); // this makes a safe copy of the buffer !!!
    log::debug!("hget: cstring::url={:?}", url);

    let get_fn = move || {
        let rt = Runtime::new();
        if let Err(err) = rt {
            log::error!("hget: could not create tokio runtime: {}", err);
            return;
        }
        let rt = rt.unwrap();

        log::debug!("hget: block_on");
        rt.block_on(async move {
            log::debug!("hget: cstr::url at {:?}", url.as_ptr());
            let url = url.to_str();
            let url = url.unwrap();
            log::debug!("hget: calling http_get with {:#?}", url);
            http_get(url, |msg: &CString| { 
                let s = msg.as_ptr();
                unsafe { cb(s); }
            }).await;
            log::debug!("hget: http_get returned");
        });
        log::debug!("hget: block_on finished");
    };

    if wait {
        get_fn();
    } else {
        std::thread::spawn(get_fn);
    }
}
