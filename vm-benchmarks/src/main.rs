mod modules;

use log;
use std::{env, ffi::CString, os::raw::c_char};
use vm;

#[repr(C)]
struct ShellConfig {
    assets_path: *const c_char,
    window_title: *const c_char,
    window_width: i32,
    window_height: i32,
}

// #[link(name = "sdl2_shell")]
extern "C" {
    fn sdl2shell_run(data: ShellConfig);
}

pub fn main() {
    env_logger::init();

    let path = env::current_exe().unwrap();

    // TODO(sysint64): Just for debug purposes
    let assets_path = path
        .as_path()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets");

    let assets_path_cstring = CString::new(assets_path.to_str().unwrap()).unwrap();

    let title = CString::new("VM Benchmarks").unwrap();
    let config = ShellConfig {
        assets_path: assets_path_cstring.as_ptr(),
        window_title: title.as_ptr(),
        window_width: 1024,
        window_height: 768,
    };

    log::info!("Assets path: {}", assets_path.to_str().unwrap());
    unsafe { vm::init() };
    log::info!("Successfully initialized virtual machine");

    vm::register_module(Box::new(modules::quads::QuadsModule::new()));
    log::info!("Successfully registered quads module");

    log::info!("Run shell");
    unsafe { sdl2shell_run(config) };

    log::info!("Shutdown");
}
