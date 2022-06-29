# sense-hat 

Raspberry PI sense hat in rust.

## Requirements
Install aarch64/arm7 targets:
```
rustup add target aarch64-unknown-linux-musleabihf
rustup add target arm7-unknown-linux-musleabihf
```

Install linkers:
```
brew install FiloSottile/musl-cross/musl-cross --without-x86_64 --with-arm-hf --with-aarch64
```

Update cargo config file: ~/.cargo/config
```
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-musleabihf-ld"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-ld"
```


## Cross building for raspberry pi aarch64
Cross building for aarch64.
```
cargo build --target=aarch64-unknown-linux-musl --release
``` 

copy the build to the raspberry pi.
```
scp -i ~/.ssh/pi_rsa  target/aarch64-unknown-linux-musl/release/sense-hat pi@192.168.1.7:~/
```
