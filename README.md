# rp2040_loader

Create a web server and writes the posted file to an attached rp2040 pico.

```
rustup target add x86_64-pc-windows-gnu
sudo apt update
sudo apt-get install mingw-w64
```

```
curl -X POST -F "firmware=@/etc/passwd" http://host.docker.internal:3000/upload
```