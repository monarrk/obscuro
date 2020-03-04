# Obscuro
"Obscuro - Algo que es confuso o difícil de entener"

A WIP kernel.

### Credit
This kernel, especially in the beginning, uses an extremely large amount of code from [this awesome tutorial](https://os.phil-opp.com).

### Building 
Dependancies:
 - Cargo xbuild
 - Cargo bootimage
 - Grub2 (grub-mkrescue)
 - Qemu?

To build, use `sys/x.sh` (if you want to reverse this, you can use `sys/delete.sh`).

Currently, while this builds a cdrom, the iso is invalid because it isn't multiboot. Hopefully this will change *very* soon.

### Testing
This has been tested with Qemu. To run, use `qemu-system-x86_64 iso/boot/oImage.bin`
