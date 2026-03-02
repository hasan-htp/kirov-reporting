# kirov-reporting

## install python dependencies
Debian:
```
sudo apt-get install ffmpeg
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
During the generation, the chip type will be asked choose ESP32S3 chip

suprizingly, all what you need to do is to connect your esp32 to a usb port and run `cargo run` 

you should see somthing like this in `lsusb` command output:
```
Bus 003 Device 020: ID 303a:1001 Espressif USB JTAG/serial debug unit
```
you might need to reset the device by long pressing the reset button 

if you get error opening the serial port it is most likely permession problem run

```
sudo usermod -aG dialout $USER
```
