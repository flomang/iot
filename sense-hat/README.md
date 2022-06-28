# sense-hat 

Raspberry PI sense hat in rust.

## Requirements
Install cross:
```
cargo install cross --git https://github.com/cross-rs/cross
```
[*cross*](https://github.com/cross-rs/cross)


## Cross building for raspberry pi aarch64
Cross building using default target.
```
cross build --target=aarch64-unknown-linux-musl 
``` 

copy the build to the raspberry pi.
```
scp -i ~/.ssh/pi_rsa  target/aarch64-unknown-linux-musl/debug/sense-hat pi@192.168.1.7:~/
```

note: cross doesn't work for M1 macs currently: 06-28/2022. Build using rustup target.

```
rustup add target armv7-unknown-linux-musleabihf
cargo build --release --target armv7-unknown-linux-musleabihf
scp -i ~/.ssh/pi_rsa  target/armv7-unknown-linux-musleabihf/release/sense-hat pi@192.168.1.7:~/
```
