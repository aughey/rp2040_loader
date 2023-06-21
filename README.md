# rp2040_loader

Create a web server and writes the posted file to an attached rp2040 pico.

The objective is to utilize Devcontainer for constructing firmware for an RP2040, employing a host machine that runs on either Windows or Linux natively. This software operates on the host machine and keeps track of the drive that is recognized when the RP2040 is connected. Whenever a file is uploaded to the web server, the software will write this file to the RP2040.

```
rustup target add x86_64-pc-windows-gnu
sudo apt update
sudo apt-get install mingw-w64
```

```
cargo build --release && elf2uf2-rs ./target/thumbv6m-none-eabi/release/rp2040-project-template && curl -X POST -F "firmware=@target/thumbv6m-none-eabi/release/rp2040-project-template.uf2" http://host.docker.internal:3000/upload
```