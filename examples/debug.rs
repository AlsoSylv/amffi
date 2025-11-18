use amffi::core::version::AMF_VERSION;

extern crate amffi;

fn main() {
    let lib = amffi::amf_init().unwrap();
    let factory = lib.init_factory(AMF_VERSION).unwrap();
    let debug = factory.get_debug().unwrap();
    println!("asserts_enabled: {}", debug.asserts_enabled());
    debug.asserts_enable(true);
    println!("asserts_enabled: {}", debug.asserts_enabled());
}