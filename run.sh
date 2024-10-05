#!/bin/bash
rust-objcopy --output-target binary target/aarch64-unknown-none/release/my_os my_os.o
qemu-system-aarch64 -m 4096 -cpu cortex-a76 -M virt -serial mon:stdio -nographic -device loader,addr=0x40800000,cpu-num=0,file=my_os.o