#!/bin/bash

# Build our kernel first of all
echo "Building kernel..."
if [ "$1" == "debug" ]; then
    cd kernel && cargo build && cd ..
else 
    cd kernel && cargo build --release && cd ..
fi
echo "Done!"
# Time to put together an iso!
# Remove root dir if there was one before
echo "Creating image..."
rm -rf image_dir
mkdir -p image_dir/boot
if [ "$1" == "debug" ]; then
    cp -v kernel/target/x86_64-blames/debug/blames image_dir/boot
else 
    cp -v kernel/target/x86_64-blames/release/blames image_dir/boot
fi
mkdir -p image_dir/boot/limine

echo "Downloading limine bootloader"
git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
make -C limine
echo "Installing limine bootloader to iso"
cp limine.conf limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin image_dir/boot/limine
mkdir -p image_dir/EFI/BOOT
cp -v limine/BOOTX64.EFI image_dir/EFI/BOOT/
cp -v limine/BOOTIA32.EFI image_dir/EFI/BOOT/
xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
	--efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
	image_dir -o blames.iso
./limine/limine bios-install blames.iso
rm -rf image_dir
