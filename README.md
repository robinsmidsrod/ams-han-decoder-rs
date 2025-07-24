# AMS-HAN-decoder implemented in Rust

Decoder of binary packet data coming from power meters in Norway using the AMS HAN port.

Use https://github.com/robinsmidsrod/ser2tcp to make the serial port data available over a TCP port usable by this utility.

## Building

```bash
cargo build
```

## Installing

 ```bash
 cargo install --path .
 ```
