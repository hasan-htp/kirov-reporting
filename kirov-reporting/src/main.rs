use std::sync::atomic::{AtomicBool, Ordering};
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2s::config::StdConfig;
use esp_idf_svc::hal::i2s::config::DataBitWidth;
use esp_idf_svc::hal::i2s::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;
use std::time::Duration;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

// https://files.waveshare.com/wiki/common/ES8311.user.Guide.pdf
fn es8311_init(i2c: &mut I2cDriver) -> Result<(), Box<dyn std::error::Error>> {

    const ES8311_ADDR: u8 = 0x18;

    i2c.write(ES8311_ADDR,&[0x00, 0x80],1000)?; // reset reg, csm power om
    i2c.write(ES8311_ADDR,&[0x01, 0x95],1000)?; // clk maanager reg, from bclk, mlk offm bclk on, reset master, reset dac
    i2c.write(ES8311_ADDR,&[0x02, 0x18],1000)?; // clk maanager reg, mclk_prediv = 3
    i2c.write(ES8311_ADDR,&[0x0D, 0x02],1000)?; // enable analog ADC and DAC, disable internal ref and normal vmid operation,
    i2c.write(ES8311_ADDR,&[0x12, 0x00],1000)?; // power up dac
    i2c.write(ES8311_ADDR,&[0x13, 0x10],1000)?; // enable output to HP drive
    i2c.write(ES8311_ADDR,&[0x32, 0xBF],1000)?; // dac volume level (think about make it adjustable !)

    // -95.5dB to +32d in 0.5dB per step:
    // 0xBF -> 0dB
    // 0xC0 -> 0.5dB
    // 0xFF +32dB

    Ok(())
}

fn play_record(sound : &[u8], i2s :&mut I2sDriver<I2sTx>, amp_enable :&mut PinDriver<Gpio18,Output>) -> Result<(), Box<dyn std::error::Error>>
{
    const CHUNK_SIZE : usize = 4096;
    let mut offset: usize = 0;

    amp_enable.set_high()?;
    std::thread::sleep(Duration::from_millis(200));
    
    while offset < sound.len() {
        let end = (offset + CHUNK_SIZE).min(sound.len());
        i2s.write(&sound[offset..end], 1000)?;
        offset = end;
    }

    amp_enable.set_low()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();


    const SOUNDS: &[&[u8]] = &[
        include_bytes!(env!("SOUND_0")),
        include_bytes!(env!("SOUND_1")),
        include_bytes!(env!("SOUND_3")),
        include_bytes!(env!("SOUND_4")),
        include_bytes!(env!("SOUND_5")),
        include_bytes!(env!("SOUND_6")),
        include_bytes!(env!("SOUND_7")),
        include_bytes!(env!("SOUND_8")),
        include_bytes!(env!("SOUND_9")),
        include_bytes!(env!("SOUND_10")),
    ];

    const MAX_COUNT: usize = SOUNDS.len();

    let peripherals = Peripherals::take().unwrap();

    let mut amp_enable = PinDriver::output(peripherals.pins.gpio18)?;

    let i2c_config = I2cConfig::new().baudrate(400.kHz().into());
    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio45,
        peripherals.pins.gpio0,
        &i2c_config,
    )?;

    es8311_init(&mut i2c)?;

    let std_config = StdConfig::philips(48000, DataBitWidth::Bits16);
    let mut i2s = I2sDriver::new_std_tx(
        peripherals.i2s0,
        &std_config,
        peripherals.pins.gpio17, // BLCLK/SLCLK
        peripherals.pins.gpio48,
        None::<AnyIOPin>,         // no MCLK pin
        peripherals.pins.gpio3,
    )?;
    i2s.tx_enable()?;

    // user button on Atom_EchoS3R is GPIO41, has a pull up resistor
    let mut button = PinDriver::input(peripherals.pins.gpio41)?;
    button.set_pull(Pull::Up)?;
    button.set_interrupt_type(InterruptType::NegEdge)?;

    let mut count: usize = 0;
    unsafe {
        button.subscribe(|| {
            BUTTON_PRESSED.store(true, Ordering::Relaxed);
        })?;
    }

    button.enable_interrupt()?;

    play_record(SOUNDS[0], &mut i2s, &mut amp_enable)?;

    loop {
        if BUTTON_PRESSED.load(Ordering::Relaxed) {
            BUTTON_PRESSED.store(false, Ordering::Relaxed);

            play_record(SOUNDS[count], &mut i2s, &mut amp_enable)?;

            count += 1;
            if count >= MAX_COUNT { 
                count=0;
            }
            button.enable_interrupt()?;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}