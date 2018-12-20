
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

mod fs;
mod implementation;
mod store;
pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}

extern "C" {
    fn main_cpp(app: *const ::std::os::raw::c_char);
}

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    use std::ffi::CString;
    let app_name = ::std::env::args().next().unwrap();
    let app_name = CString::new(app_name).unwrap();
    unsafe {
        main_cpp(app_name.as_ptr());
    }
}
