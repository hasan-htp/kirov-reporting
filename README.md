# kirov-reporting

## install python dependencies
Debian:
```
sudo apt-get install ffmpeg
sudo apt-get install portaudio19-dev
```
create python env:
```
python3 -m venv venv
```
source the env:
```
source venv/bin/activate
```

install the following dependencies:

```
pip install audio2numpy
pip install sounddevice
```

## install rust and ESP32 dependencies

```
cargo install espup --locked 
```

```
espup install --targets esp32
```

source the env:
```
source /home/${USER}/export-esp.sh
```

```
cargo install espflash --locked
cargo install ldproxy --locked
cargo install cargo-generate --locked
cargo install cargo-espflash --locked
```

```
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```
During the generation, the chip type will be asked, select ESP32S3 chip

surprisingly, all what you need to do is to connect your esp32 to a usb port and run `cargo run` 

you should see somtehing like this in `lsusb` command output:
```
Bus 003 Device 020: ID 303a:1001 Espressif USB JTAG/serial debug unit
```
you might need to reset the device by long pressing the reset button 

if you get error opening the serial port it is most likely permession problem, run

```
sudo usermod -aG dialout $USER
```


## Resources
https://docs.m5stack.com/en/core/Atom_EchoS3R

https://github.com/esp-rs/esp-idf-hal/tree/master