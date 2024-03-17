fn main() {
    let application_processor_boot_loader_base: &str = option_env!("APPLICATION_PROCESSOR_BOOT_LOADER_BASE").unwrap();
    let application_processor_boot_loader_stack_floor: &str = option_env!("APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR").unwrap();
    println!("application_processor_boot_loader_base = {:#x?}", application_processor_boot_loader_base);
    println!("application_processor_boot_loader_stack_floor = {:#x?}", application_processor_boot_loader_stack_floor);
}

