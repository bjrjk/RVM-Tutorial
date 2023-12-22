mod hal;

use rvm::RvmPerCpu;

use self::hal::RvmHalImpl;

pub fn run() {
    println!("Starting virtualization...");
    println!("Hardware support: {:?}", rvm::has_hardware_support());

    rvm::print_CPU_info();

    let mut percpu = RvmPerCpu::<RvmHalImpl>::new(0);
    println!("The VMX state before VMX enabled: {}", percpu.is_enabled());
    percpu.hardware_enable().unwrap();
    println!("The VMX state after VMX enabled: {}", percpu.is_enabled());

    println!("This is Hello, World from VMX Root Mode.");

    let mut vcpu = percpu.create_vcpu().unwrap();
    info!("{:#x?}", vcpu);
    vcpu.run();
}
