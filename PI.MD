## Cross compile for PI1
```
sudo apt install symlinks
./sync.sh
./b.sh
```
## Install cross compiler 
sudo apt install arm-none-linux-leabihf
## sync directories from PI
Make sure that all necessary LVGL GUI dependency libraries are installed on PI before copying sysroot 
- wayland : wayland-client wayland-cursor wayland-protocols xkbcommon
- framebuffer :

