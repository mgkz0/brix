cargo +nightly build --release --target x86_64-unknown-none

cp target/x86_64-unknown-none/release/brix iso/boot/brix.elf

xorriso -as mkisofs \
    -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part \
    --efi-boot-image \
    --protective-msdos-label \
    iso -o brix.iso

qemu-system-x86_64 \
    -machine q35 \
    -m 512M \
    -cdrom brix.iso \
    -display gtk,zoom-to-fit=on \
    -boot d
