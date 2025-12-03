use amffi::core::{
    trace::{AMF_TRACE_WRITER_CONSOLE, AMF_TRACE_WRITER_DEBUG_OUTPUT},
    version::AMF_VERSION,
};

extern crate amffi;

fn main() {
    let lib = amffi::amf_init().unwrap();
    let factory = lib.init_factory(AMF_VERSION).unwrap();
    let debug = factory.get_debug().unwrap();
    let trace = factory.get_trace().unwrap();
    trace.set_writer_enabled(AMF_TRACE_WRITER_CONSOLE, true);
    trace.set_writer_enabled(AMF_TRACE_WRITER_DEBUG_OUTPUT, true);

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

    trace
        .trace_w(
            amffi::core::trace::AMFTraceLevel::Info,
            "AMFTraceTest",
            format!("Write something out to the enabled traces"),
        )
        .unwrap();
}
