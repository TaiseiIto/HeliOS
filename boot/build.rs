use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
};

fn main() {
    let processor_boot_loader: String = env::var("PROCESSOR_BOOT_LOADER").unwrap();
    println!("processor_boot_loader = {:#x?}", processor_boot_loader);
    let processor_kernel: String = env::var("PROCESSOR_KERNEL").unwrap();
    println!("processor_kernel = {:#x?}", processor_kernel);
    let kernel: String = env::var("KERNEL").unwrap();
    println!("kernel = {:#x?}", kernel);
    let out_dir: String = env::var("OUT_DIR").unwrap();
    println!("out_dir = {:#x?}", out_dir);
    let constants = Path::new(&out_dir).join("constants.rs");
    println!("constants = {:#x?}", constants);
    let mut constants = File::create(&constants).unwrap();
    writeln!(&mut constants, "const PROCESSOR_BOOT_LOADER: &str = {:#x?};", processor_boot_loader).unwrap();
    writeln!(&mut constants, "const PROCESSOR_KERNEL: &str = {:#x?};", processor_kernel).unwrap();
    writeln!(&mut constants, "const KERNEL: &str = {:#x?};", kernel).unwrap();
}

