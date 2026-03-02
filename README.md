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
cargo install espflash
cargo install ldproxy
cargo install cargo-generate
cargo install cargo-espflash
```

```
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```
