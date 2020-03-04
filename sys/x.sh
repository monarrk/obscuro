#!/bin/bash
function fail() {
	printf "FAILURE! Read a log in ./log.txt!\n"
	exit 1
}

echo > ./log.txt
printf "Find logs at log.txt!\n"

printf "Building..."
cargo xbuild --target $(pwd)/sys/extras/x86_64-obscuro.json || fail
printf "OK\n"

printf "Making bootimage..."
cargo bootimage --target $(pwd)/sys/extras/x86_64-obscuro.json &>> log.txt || fail
cp target/x86_64-obscuro/debug/bootimage-obscuro.bin obscuro.bin
printf "OK\n"
