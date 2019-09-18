#!/bin/bash
function fail() {
	printf "FAILURE! Read a log in ./log.txt!\n"
	exit 1
}

echo > ./log.txt
printf "Find logs at log.txt!\n"

printf "Building..."
cp sys/extras/*.json .
cargo xbuild &>> log.txt || fail
printf "OK\n"

printf "Making bootimage..."
cargo bootimage &>> log.txt || fail
printf "OK\n"

printf "Creating ISO directory..."
mkdir -p iso/boot/grub
printf "OK\n"

printf "Populating dir..."
cp target/x86_64-obscuro/debug/bootimage-obscuro.bin iso/boot/oImage.bin
cp sys/extras/grub.cfg iso/boot/grub/grub.cfg
printf "OK\n"

printf "Making ISO..."
grub-mkrescue iso -o "obscuro-0.0.1.iso" &>> log.txt || fail
printf "OK\n"
