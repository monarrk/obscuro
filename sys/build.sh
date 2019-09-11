printf "Find logs at log.txt!\n"

printf "Building..."
cargo xbuild &>> log.txt || exit 1
printf "OK\n"

printf "Making bootimage..."
cargo bootimage &>> log.txt || exit 1
printf "OK\n"

printf "Creating ISO directory..."
mkdir -p iso/boot/grub
printf "OK\n"

printf "Populating dir..."
cp target/x86_64-obscuro/debug/bootimage-obscuro.bin iso/boot/oImage.bin
cp grub.cfg iso/boot/grub/grub.cfg
printf "OK\n"

printf "Making ISO..."
grub-mkrescue iso -o "obscuro-0.0.1.iso" &>> log.txt || exit 1
printf "OK\n"
