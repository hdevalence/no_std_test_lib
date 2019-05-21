# tests using `rand_core` and `curve25519-dalek` with `no_std`

Install toolchain:
```
rustup target add thumbv7em-none-eabihf
```
Build crate
```
cargo build --release --target thumbv7em-none-eabihf --no-default-features
```

Results on my machine:
```
$ cargo build --release --target thumbv7em-none-eabihf --no-default-features
   Compiling typenum v1.10.0
   Compiling cc v1.0.37
   Compiling byteorder v1.3.1
   Compiling rand_core v0.4.0
   Compiling subtle v2.1.0
   Compiling rand_core v0.3.1
   Compiling generic-array v0.12.0
   Compiling digest v0.8.0
   Compiling clear_on_drop v0.2.3
   Compiling curve25519-dalek v1.1.4
   Compiling no_std_test_lib v0.1.0 (.../no_std_test_lib)
    Finished release [optimized] target(s) in 12.63s
```
