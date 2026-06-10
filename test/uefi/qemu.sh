qemu-system-x86_64 \
  -enable-kvm \
  -m 4G \
  -drive if=pflash,format=raw,file=OVMF_CODE.4m.fd \
  -drive if=pflash,format=raw,file=ovmf_vars.fd \
  -nographic
