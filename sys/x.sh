#!/bin/bash
function fail() {
	printf "FAILURE! Read a log in ./log.txt!\n"
	exit 1
}

printf "Making bootimage..."
cargo bootimage --target $(pwd)/sys/extras/x86_64-obscuro.json || fail
cp target/x86_64-obscuro/debug/bootimage-obscuro.bin obscuro.bin
printf "OK\n"
