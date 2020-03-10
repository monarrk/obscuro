#!/bin/bash
function fail() {
	printf "FAILURE!\n"
	exit 1
}

printf "Making bootimage..."
cargo bootimage --target $(pwd)/sys/extras/x86_64-obscuro.json || fail
cp target/x86_64-obscuro/debug/bootimage-obscuro.bin obscuro.bin
printf "OK\n"

printf "Use sys/test.sh to test the kernel or burn obscuro.bin to a flash drive\n"
