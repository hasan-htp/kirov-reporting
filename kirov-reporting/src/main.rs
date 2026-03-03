
use std::sync::atomic::{AtomicBool, Ordering};
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::prelude::*;
use std::time::Duration;


static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
const MAX_COUNT :u32 = 11; // records count

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    // user button on Atom_EchoS3R is GPIO41, has a pull up resistor
    let mut button = PinDriver::input(pins.gpio41)?;
    button.set_pull(Pull::Up)?;
    button.set_interrupt_type(InterruptType::NegEdge)?;

    let mut count = 0;
    unsafe {
        button.subscribe(|| {
            BUTTON_PRESSED.store(true, Ordering::Relaxed);
        })?;
    }

    button.enable_interrupt()?;

    loop {
        if BUTTON_PRESSED.load(Ordering::Relaxed) {
            BUTTON_PRESSED.store(false, Ordering::Relaxed);
            println!("Kirov Reporting {}", count);
            count+=1;
            if count >= MAX_COUNT
            {
                count =0;
            }
            std::thread::sleep(Duration::from_millis(10));
            button.enable_interrupt()?;

        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
