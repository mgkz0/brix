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
  -display gtk,zoom-to-fit=on \
  -drive format=raw,file=fat:rw:esp \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd
