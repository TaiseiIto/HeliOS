use std::env;

fn main() {
    let application_processor_boot_loader_base: String = env::var("APPLICATION_PROCESSOR_BOOT_LOADER_BASE").unwrap();
    let application_processor_boot_loader_stack_floor: String = env::var("APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR").unwrap();
    let out_dir: String = env::var("OUT_DIR").unwrap();
    println!("application_processor_boot_loader_base = {:#x?}", application_processor_boot_loader_base);
    println!("application_processor_boot_loader_stack_floor = {:#x?}", application_processor_boot_loader_stack_floor);
    println!("out_dir = {:#x?}", out_dir);
}

