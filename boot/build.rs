use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
};

fn main() {
    let application_processor_boot_loader: String = env::var("APPLICATION_PROCESSOR_BOOT_LOADER").unwrap();
    println!("application_processor_boot_loader = {:#x?}", application_processor_boot_loader);
    let application_processor_boot_loader_base: String = env::var("APPLICATION_PROCESSOR_BOOT_LOADER_BASE").unwrap();
    println!("application_processor_boot_loader_base = {:#x?}", application_processor_boot_loader_base);
    let application_processor_boot_loader_stack_floor: String = env::var("APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR").unwrap();
    println!("application_processor_boot_loader_stack_floor = {:#x?}", application_processor_boot_loader_stack_floor);
    let kernel: String = env::var("KERNEL").unwrap();
    println!("kernel = {:#x?}", kernel);
    let out_dir: String = env::var("OUT_DIR").unwrap();
    println!("out_dir = {:#x?}", out_dir);
    let constants = Path::new(&out_dir).join("constants.rs");
    println!("constants = {:#x?}", constants);
    let mut constants = File::create(&constants).unwrap();
    writeln!(&mut constants, "const APPLICATION_PROCESSOR_BOOT_LOADER_BASE: usize = {};", application_processor_boot_loader_base).unwrap();
    writeln!(&mut constants, "const APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR: usize = {};", application_processor_boot_loader_stack_floor).unwrap();
}

