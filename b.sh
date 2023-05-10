PI_IP=192.168.0.242 # Be sure to change this!
PI_USER=lieven
PI_HOST=pi1.local
export SSHPASS=mg61dd
#TARGET=armv7-unknown-linux-gnueabihf # Pi 2/3/4
#TARGET=arm-unknown-linux-gnueabihf # Pi 0/1
TARGET=arm-unknown-linux-gnueabi
# TOOLCHAIN=armv6-linux-musleabihf
TOOLCHAIN=arm-none-linux-gnueabihf

# exec PKG_CONFIG_EXECUTABLE=${TRIPLE}-pkg-config
export PKG_CONFIG_PATH=$HOME/rpi-sysroot
export PKG_CONFIG_SYSROOT_DIR=$HOME/rpi-sysroot
export CC=$TOOLCHAIN-gcc

set -x
PATH="$HOME/toolchain/$TOOLCHAIN/bin:/usr/bin:/$HOME/.cargo/bin"
#export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-none-linux-gnueabihf-gcc
export CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER=$TOOLCHAIN-gcc
# build binary
# cargo clean
cargo build --release --target $TARGET
# cargo build --target $TARGET
# upload binary
sshpass -e  rcp  ./target/$TARGET/release/pi_project $PI_HOST:.
sshpass -e  rcp  ./target/$TARGET/release/pi_project pi2.local:.
## execute binary
#sshpass -e ssh $PI_HOST './pi_project'
