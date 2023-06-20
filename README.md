# rp2040_loader

Create a web server and writes the posted file to an attached rp2040 pico.

The goal is to use devcontainer to build firmware for an RP2040 with the host machine
being a native Windows or Linux machine.  This application runs on the host machine and monitors
the drive that will exist when the RP2040 is plugged in.  When a file is posted to the web server
it will write the file to the RP2040.

```
rustup target add x86_64-pc-windows-gnu
sudo apt update
sudo apt-get install mingw-w64
```

```
curl -X POST -F "firmware=@/etc/passwd" http://host.docker.internal:3000/upload
```