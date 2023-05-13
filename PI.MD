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
- choose pi1 and rename to arm-linux-gnueabihf