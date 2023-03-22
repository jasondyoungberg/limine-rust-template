#!/bin/sh
#
# This script will be executed by `cargo run`.

set -ex

# Cargo passes the path to the built executable as an argument.
KERNEL=$1

LIMINE_GIT_URL="https://github.com/limine-bootloader/limine.git"

# Clone/update Limine if needed.
if [ ! -d limine ]; then
    git clone $LIMINE_GIT_URL --depth=1 --branch=v3.0-branch-binary limine
fi
cd limine
git fetch
make
cd -

# Create a directory for the ISO and copy all the needed files.
mkdir -p target/iso_root
cp limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin conf/limine.cfg target/iso_root/
cp "$KERNEL" target/iso_root/kernel.elf

# Create a bootable ISO image from the directory.
xorriso -as mkisofs                                             \
    -b limine-cd.bin                                            \
    -no-emul-boot -boot-load-size 4 -boot-info-table            \
    --efi-boot limine-cd-efi.bin                                \
    -efi-boot-part --efi-boot-image --protective-msdos-label    \
    target/iso_root -o target/barebones.iso

# Install Limine onto the image.
limine/limine-deploy barebones.iso

# Run it!
qemu-system-x86_64                                              \
    -machine q35 -cpu qemu64 -M smm=off                         \
    -D target/qemu-log.txt -d guest_errors,int -no-reboot     \
    -no-shutdown -serial mon:stdio barebones.iso
