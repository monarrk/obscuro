#!/bin/sh

qemu-system-x86_64 -drive format=raw,file=obscuro.bin -serial stdio
