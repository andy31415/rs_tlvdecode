## Simple decoder of CHIP TLV values

Example usage:


```
git clone https://github.com/andy31415/rs_tlvdecode.git
cd rs_tlvdecode
cargo run -- --hex 153600171818290324FF0118
```


Or compiling things:

```
cargo build --release

./target/release/tlvdecode --hex 153600171818290324FF0118
```