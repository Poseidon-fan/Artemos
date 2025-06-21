rm -rf ext4.img
dd if=/dev/zero of=ext4.img bs=1M count=256
mkfs.ext4 ./ext4.img