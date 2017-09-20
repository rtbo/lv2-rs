#! /bin/sh

LV2_DIR=$HOME/.lv2
CONFIG=debug

function install_plugin {
    cargo build --manifest-path plugins/$1/Cargo.toml
    rm -vrf $LV2_DIR/$1.lv2
    mkdir -vp $LV2_DIR/$1.lv2
    cp -v plugins/$1/*.ttl $LV2_DIR/$1.lv2
    cp -v plugins/$1/target/$CONFIG/lib$1.so $LV2_DIR/$1.lv2/$1.so
}

# install_plugin amp_ffi
install_plugin amp
