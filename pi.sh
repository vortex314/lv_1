set -x
PI_IP=192.168.0.242 # Be sure to change this!
PI_HOST=pi1.local
TARGET=arm-unknown-linux-gnueabihf # Pi 0/1
TOOLCHAIN=arm-linux-gnueabihf
TOOLCHAIN_DIR=$HOME/toolchain/$TOOLCHAIN
export SYSROOT="$HOME/rpi-sysroot"   # location of the Pi includes and libs
export DEP_LV_CONFIG_PATH=`pwd`/lv_pi      # location of the lvgl config file lv_conf.h lv_drv_conf.h
export CC=$TOOLCHAIN-gcc
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=$TOOLCHAIN-gcc
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-C linker=$TOOLCHAIN-gcc"
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
export LVGL_LINK="gtk-3,gdk-3,pangocairo-1.0,pango-1.0,harfbuzz,atk-1.0,cairo-gobject,cairo,gdk_pixbuf-2.0,gio-2.0,gobject-2.0,glib-2.0,z,gmodule-2.0,selinux,pcre2-8,mount,resolv,ffi,pcre,blkid"
# export LIBCLANG_PATH=`llvm-config --libdir`
export CC=$TOOLCHAIN_DIR/bin/$TOOLCHAIN-gcc
export CFLAGS="--sysroot=$SYSROOT -fPIC -march=armv6 -marm -mfpu=vfp -gdwarf-4 -fno-omit-frame-pointer"
export CFLAGS_arm_linux_gnueabihf="--sysroot=$SYSROOT -fPIC -march=armv6 -marm -mfpu=vfp -gdwarf-4 -fno-omit-frame-pointer"
export CRATE_CC_NO_DEFAULTS=1
export PATH="$TOOLCHAIN_DIR/bin:/usr/bin:$HOME/.cargo/bin"
export LDFLAGS="--sysroot=$SYSROOT -L$SYSROOT/usr/lib/arm-linux-gnueabihf:$SYSROOT/lib/arm-linux-gnueabihf:$TOOLCHAIN_DIR/$TOOLCHAIN/libc/usr/lib"
export LD_LIBRARY_PATH="$SYSROOT/usr/lib/arm-linux-gnueabihf:$SYSROOT/lib/arm-linux-gnueabihf:$TOOLCHAIN_DIR/$TOOLCHAIN/libc/usr/lib"
# build binary
# cargo clean
cargo build --target $TARGET 
# cargo build --target $TARGET
# upload binary
rcp  ./target/$TARGET/debug/lv_1 $PI_HOST:.
rcp  ./target/$TARGET/debug/lv_1 pi2.local:.
