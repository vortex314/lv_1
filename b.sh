set -x
PI_IP=192.168.0.242 # Be sure to change this!
PI_HOST=pi1.local
TARGET=arm-unknown-linux-gnueabihf # Pi 0/1
TOOLCHAIN=arm-none-linux-gnueabihf
# exec PKG_CONFIG_EXECUTABLE=${TRIPLE}-pkg-config
#PKG_CONFIG_LIBDIR=/usr/lib/pkgconfig:/usr/share/pkgconfig/:${TARGET_SYSROOT}/usr/lib/aarch64-linux-gnu/pkgconfig:${TARGET_SYSROOT}/usr/lib/pkgconfig)
export SYSROOT="$HOME/rpi-sysroot"   # location of the Pi includes and libs
export DEP_LV_CONFIG_PATH=`pwd`/lv_pi      # location of the lvgl config file lv_conf.h lv_drv_conf.h
export PKG_CONFIG_PATH=$SYSROOT     
export PKG_CONFIG_SYSROOT_DIR=$SYSROOT # root from where pkg-config seeks libs and packages 
export CC=$TOOLCHAIN-gcc
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=$TOOLCHAIN-gcc
export LVGL_INCLUDE="\
$SYSROOT/usr/include/gtk-3.0,\
$SYSROOT/usr/include,\
$SYSROOT/usr/local/include,\
$SYSROOT/usr/include/glib-2.0,\
$SYSROOT/usr/lib/arm-linux-gnueabihf/glib-2.0/include,\
$SYSROOT/usr/include/arm-linux-gnueabihf,\
$SYSROOT/usr/include/pango-1.0,\
$SYSROOT/usr/include/harfbuzz,\
$SYSROOT/usr/include/cairo,\
$SYSROOT/usr/include/gdk-pixbuf-2.0,\
$SYSROOT/usr/include/atk-1.0,\
$SYSROOT/usr/include/freetype2"
export LVGL_LINK="gtk-3,gdk-3,pangocairo-1.0,pango-1.0,harfbuzz,atk-1.0,cairo-gobject,cairo,gdk_pixbuf-2.0,gio-2.0,gobject-2.0,glib-2.0"
export LIBCLANG_PATH=`llvm-config --libdir`

# export CMAKE_SYSROOT=$SYSROOT
# export CMAKE_C_FLAGS="-I$SYSROOT/usr/include"
# export CMAKE_CXX_FLAGS="-I$SYSROOT/usr/include"
# export CMAKE_PREFIX_PATH=$SYSROOT
# export CMAKE_LIBRARY_PATH=$SYSROOT/usr/lib/arm-linux-gnueabihf
# export CC=$TOOLCHAIN-gcc
export CFLAGS="-fPIC -L$SYSROOT/usr/lib -I$SYSROOT/usr/include -pthread -lpthread"
export CRATE_CC_NO_DEFAULTS=1

PATH="$HOME/toolchain/$TOOLCHAIN/bin:/usr/bin:/$HOME/.cargo/bin"
# build binary
# cargo clean
cargo build --target $TARGET 
# cargo build --target $TARGET
# upload binary
# sshpass -e  rcp  ./target/$TARGET/release/pi_project $PI_HOST:.
# sshpass -e  rcp  ./target/$TARGET/release/pi_project pi2.local:.
## execute binary
#sshpass -e ssh $PI_HOST './pi_project'
