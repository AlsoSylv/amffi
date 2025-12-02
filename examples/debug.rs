use amffi::core::version::AMF_VERSION;

extern crate amffi;

fn main() {
    let lib = amffi::amf_init().unwrap();
    let factory = lib.init_factory(AMF_VERSION).unwrap();
    let debug = factory.get_debug().unwrap();
    debug.asserts_enable(false);
    println!("asserts_enabled: {}", debug.asserts_enabled());
    debug.asserts_enable(true);
    println!("asserts_enabled: {}", debug.asserts_enabled());

    println!(
        "performance_monitor: {}",
        debug.performance_monitor_enabled()
    );
    debug.enable_performance_monitor(true);
    println!("asserts_enabled: {}", debug.performance_monitor_enabled());
}
