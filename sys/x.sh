#!/bin/sh
fail() {
	printf "FAILURE!\n"
	exit 1
}

printf "Making bootimage..."
cargo bootimage --target $(pwd)/sys/extras/x86_64-obscuro.json --release || fail
cp target/x86_64-obscuro/release/bootimage-obscuro.bin ./obscuro.bin
printf "OK\n"

printf "Use sys/test.sh to test the kernel or burn obscuro.bin to a flash drive\n"
