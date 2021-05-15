# STM32 kernel task program

```
cargo rustc -- --emit=obj --verbose
cargo build --verbose 
cp ./target/thumbv7em-none-eabi/debug/cs525-kernel-task1 ./

objdump -h cs525-kernel-task1


// to compile multiple different tasks
// 1. change memory_task#.x in link.x
INCLUDE memory_task2.x
// 2.
cargo rustc --bin task2 -- --emit=obj --verbose
// 3.
cargo build --bin task2
// 4.
cp ./target/thumbv7em-none-eabi/debug/task2 ./cs525-kernel-task2
// 5. 
objdump -h cs525-kernel-task1
```