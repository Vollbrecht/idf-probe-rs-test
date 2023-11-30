use esp_idf_svc::hal::delay::FreeRtos;
use rtt_target::{rprintln, rtt_init_print};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    rtt_init_print!();

    let mut counter = 0;
    loop {
        FreeRtos::delay_ms(500);
        rprintln!("RTT, {}!", counter);
        counter += 1;
    }
}
