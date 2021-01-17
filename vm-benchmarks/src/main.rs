mod modules;

use std::env;
use vm;

#[link(name = "sdl2_shell")]
extern "C" {
    fn sdl2shell_run();
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", env::current_dir().unwrap().into_os_string());

    println!("Initialize");
    unsafe { vm::init() };

    println!("Register quads module");
    vm::register_module(Box::new(modules::quads::QuadsModule::new()));

    println!("Run shell");
    unsafe { sdl2shell_run() };

    println!("Shutdown");
}
