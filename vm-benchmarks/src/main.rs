mod modules;

use std::{env, ffi::CString, os::raw::c_char};
use vm;

#[repr(C)]
struct ShellConfig {
    assets_path: *const c_char,
    window_title: *const c_char,
    window_width: i32,
    window_height: i32,
}

#[link(name = "sdl2_shell")]
extern "C" {
    fn sdl2shell_run(data: ShellConfig);
}

pub fn main() {
    let path = env::current_exe().unwrap();
    let assets_path = CString::new(
        path.as_path()
            .parent()
            .unwrap()
            .join("assets")
            .to_str()
            .unwrap(),
    )
    .unwrap();
    let title = CString::new("VM Benchmarks").unwrap();
    let args: Vec<String> = env::args().collect();
    let data = ShellConfig {
        assets_path: assets_path.as_ptr(),
        window_title: title.as_ptr(),
        window_width: 1024,
        window_height: 768,
    };

    println!("{:?}", args);
    println!("{:?}", env::current_dir().unwrap());
    println!("{:?}", env::current_exe().unwrap());

    println!("Initialize");
    unsafe { vm::init() };

    println!("Register quads module");
    vm::register_module(Box::new(modules::quads::QuadsModule::new()));

    println!("Run shell");
    unsafe { sdl2shell_run(data) };

    println!("Shutdown");
}
