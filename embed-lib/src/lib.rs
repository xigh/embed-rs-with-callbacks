use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::NonNull;

type FooCallback = unsafe extern "C" fn(msg: *const c_char);

#[no_mangle]
pub unsafe extern "C" fn foo(cb: Option<FooCallback>) {
    println!("in foo: 1");
    if let None = cb {
        eprintln!("foo: cb is null");
        return;
    }
    println!("in foo: 2");
    let msg = "Hello World";
    let msg = CString::new(msg);
    println!("in foo: 3");
    if let Err(err) = msg {
        eprintln!("foo: could not allocate CString: {}", err);
        return;
    }
    println!("in foo: 4");
    let msg = msg.unwrap();
    println!("in foo: 5: msg={:#?}", msg);
    let msg = msg.as_ptr();
    println!("in foo: 6: msg={:#?} *msg={:02x}", msg, *msg);
    let cb = cb.unwrap();
    println!("in foo: 7: cb={:#?}", cb);
    cb(msg);
    println!("in foo: 8");
}


#[no_mangle]
pub unsafe extern "C" fn bar(name: Option<NonNull<c_char>>, cb: Option<FooCallback>) {
    if let None = cb {
        eprintln!("bar: cb is null");
        return;
    }
    if let None = name {
        eprintln!("hello: name is null");
        return;
    }
    let name = name.unwrap().as_ptr();
    let name = CStr::from_ptr(name);
    let msg = format!("Hello {:?}", name);
    let msg = CString::new(msg);
    if let Err(err) = msg {
        eprintln!("bar: could not allocate CString: {}", err);
        return;
    }
    let msg = msg.unwrap();
    let msg = msg.as_ptr();
    let cb = cb.unwrap();
    cb(msg);
}

#[no_mangle]
pub unsafe extern "C" fn hello(name: Option<NonNull<c_char>>) {
    if let None = name {
        eprintln!("hello: name is null");
        return;
    }
    let name = name.unwrap().as_ptr();
    let name = CStr::from_ptr(name);
    println!("hello \"{}\" from rust", name.to_str().unwrap_or("<invalid name>"))
}
