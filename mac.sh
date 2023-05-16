set -x
# TOOLCHAIN=arm-linux-gnueabihf
export DEP_LV_CONFIG_PATH=`pwd`/lv_sdl           # location of the lvgl config file lv_conf.h lv_drv_conf.h
# export LVGL_LINK="SDL,SDL2"
# export LIBCLANG_PATH="/Library/Developer/CommandLineTools/usr/lib/"
cargo clean
cargo build  -j 4

