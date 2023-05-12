TARGET=arm-unknown-linux-gnueabi
TOOLCHAIN=armv6-linux-musleabihf
export PKG_CONFIG_PATH=$HOME/rpi-sysroot
export PKG_CONFIG_SYSROOT_DIR=$HOME/rpi-sysroot
export CC=$TOOLCHAIN-gcc
set -x
PATH="$HOME/toolchain/$TOOLCHAIN/bin:/usr/bin:/$HOME/.cargo/bin"
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=$TOOLCHAIN-gcc
cargo build --release --target $TARGET --verbose

