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

## install rust dependencies

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
