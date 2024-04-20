use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
};

fn main() {
    let processor_boot_loader: String = env::var("PROCESSOR_BOOT_LOADER").unwrap();
    println!("processor_boot_loader = {:#x?}", processor_boot_loader);
    let processor_boot_loader_base: String = env::var("PROCESSOR_BOOT_LOADER_BASE").unwrap();
    println!("processor_boot_loader_base = {:#x?}", processor_boot_loader_base);
    let processor_boot_loader_stack_floor: String = env::var("PROCESSOR_BOOT_LOADER_STACK_FLOOR").unwrap();
    println!("processor_boot_loader_stack_floor = {:#x?}", processor_boot_loader_stack_floor);
    let kernel: String = env::var("KERNEL").unwrap();
    println!("kernel = {:#x?}", kernel);
    let out_dir: String = env::var("OUT_DIR").unwrap();
    println!("out_dir = {:#x?}", out_dir);
    let constants = Path::new(&out_dir).join("constants.rs");
    println!("constants = {:#x?}", constants);
    let mut constants = File::create(&constants).unwrap();
    writeln!(&mut constants, "const PROCESSOR_BOOT_LOADER: &str = {:#x?};", processor_boot_loader).unwrap();
    writeln!(&mut constants, "const PROCESSOR_BOOT_LOADER_BASE: usize = {};", processor_boot_loader_base).unwrap();
    writeln!(&mut constants, "const PROCESSOR_BOOT_LOADER_STACK_FLOOR: usize = {};", processor_boot_loader_stack_floor).unwrap();
    writeln!(&mut constants, "const KERNEL: &str = {:#x?};", kernel).unwrap();
}

