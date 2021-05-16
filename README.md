# Accelerometer Tester
Testing the `LIS2DW12` accelerometer in Rust

## Setup

The following hardware is used:

- STM32 bluepill (an STM32F103C8T6 MCU).
- ST-LINK-V2 for programming and debug logging
- STEVAL-MKI179V1 evaluation board which exposes the the `LIS2DW12` sensor

You will need [`probe-run`](https://ferrous-systems.com/blog/probe-run/) which is a utility to enable `cargo run` to run embedded applications on a device:

```
cargo install probe-run
```

![image](./img/accelerometer_with_bluepill.jpg)

Connect the following pins for SPI:
```
Bluepill -> STEVAL-MKI179V1 Eval board:

A7  -> SDA  (MOSI)
A6  -> SAO  (MISO)
A5  -> SCL  (Clock)
A4  -> CS   (Chip Select)
3.3 -> VDD  (3.3v power)
G   -> GND  (Ground)
```

## Running

```
cargo run
```

You should see output like this:
```
bla@blabla:~/source/accelerometer-test$ cargo run
   Compiling accelerometer-test v0.1.1 (/home/bla/source/accelerometer-test)
    Finished dev [optimized + debuginfo] target(s) in 0.39s
     Running `probe-run --chip STM32F103CB target/thumbv7m-none-eabi/debug/accelerometer-test`
  (HOST) INFO  flashing program (47.59 KiB)
  (HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
[INF] Initializing
[INF] Done initialising
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 372, y: -524, z: 15528 }
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 372, y: -512, z: 15532 }
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 388, y: -520, z: 15556 }
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 372, y: -528, z: 15536 }
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 380, y: -520, z: 15528 }
norm: (0.02, -0.03, 0.95), sample_rate_hz: 100, raw: I16x3 { x: 352, y: -524, z: 15516 }
```