# STM32 kernel task program

```
cargo rustc -- --emit=obj --verbose
cargo build --verbose 
cp ./target/thumbv7em-none-eabi/debug/cs525-kernel-task1 ./

objdump -h cs525-kernel-task1
```