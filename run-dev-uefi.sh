cargo +nightly build --release --target x86_64-unknown-none

cp target/x86_64-unknown-none/release/brix iso/boot/brix.elf

qemu-system-x86_64 \
    -machine q35 \
    -m 512M \
    -display gtk,zoom-to-fit=on \
    -drive format=raw,file=fat:rw:./iso \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd
