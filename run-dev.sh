#!/usr/bin/env bash
cargo build --target x86_64-unknown-uefi

if [ -f "./esp/efi/boot/bootx64.efi" ]; then
	rm -rf ./esp/efi/boot/bootx64.efi
else 
	mkdir ./esp/ && mkdir ./esp/efi && mkdir ./esp/efi/boot
fi
cp target/x86_64-unknown-uefi/debug/brix.efi \
   esp/efi/boot/bootx64.efi

qemu-system-x86_64 \
  -machine q35 \
  -m 512M \
  -drive format=raw,file=fat:rw:esp \
  -bios /usr/share/edk2/ovmf/OVMF_CODE.fd
