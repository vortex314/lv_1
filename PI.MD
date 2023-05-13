## Cross compile for PI1

https://wiki.qt.io/Cross-Compile_Qt_6_for_Raspberry_Pi
```
sudo apt install symlinks 
./sync.sh
symlinks -rc rpi-sysroot
./b.sh
```

## Install cross compiler 
sudo apt install arm-none-linux-leabihf

Looks like the default arm-toolchain doesnµt generate ARMV6 code
## sync directories from PI
Make sure that all necessary LVGL GUI dependency libraries are installed on PI before copying sysroot 
- wayland : wayland-client wayland-cursor wayland-protocols xkbcommon
- framebuffer :

## on pi
Install necessary libraries for linking and sync them to the linux PC host
```
sudo apt install ligtk-3-dev
```
### References
https://crates.io/crates/cc => C flags passed on by Rust
https://forum.qt.io/topic/106753/qt-raspberry-pi-cross-compilation-make-issue/2
https://sourceforge.net/projects/raspberry-pi-cross-compilers/postdownload 
https://wiki.qt.io/Cross-Compile_Qt_6_for_Raspberry_Pi
https://github.com/abhiTronix/raspberry-pi-cross-co

## copy libraries from Raspberry Pi to PC HOST
```
set -x
PI_USER=lieven
PI_HOST=pi1.local
cd ~
mkdir rpi-sysroot rpi-sysroot/usr rpi-sysroot/opt
rsync -avz --rsync-path="sudo rsync" --delete $PI_USER@$PI_HOST:/lib rpi-sysroot
rsync -avz --rsync-path="sudo rsync" --delete $PI_USER@$PI_HOST:/usr/include rpi-sysroot/usr
rsync -avz --rsync-path="sudo rsync" --delete $PI_USER@$PI_HOST:/usr/lib rpi-sysroot/usr
# rsync -avz --rsync-path="sudo rsync" --delete $PI_USER@$PI_HOST:/opt/vc rpi-sysroot/opt
cd
symlinks -rcd rpi-sysroot
```

## adapt symbolic links in rpi-sysroot
```
cd $SYSROOT/usr/include
ln -s arm-linux-gnueabihf/asm/ asm
ln -s arm-linux-gnueabihf/bits/ bits
ln -s arm-linux-gnueabihf/sys sys
ln -s arm-linux-gnueabihf/gnu gnu
cd ../lib
ln -s arm-linux-gnueabihf/Scrt1.o Scrt1.o
ln -s arm-linux-gnueabihf/crti.o crti.o
ln -s arm-linux-gnueabihf/crtn.o crtn.o
```