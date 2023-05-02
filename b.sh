export DEP_LV_CONFIG_PATH=`pwd`
export LVGL_INCLUDE=/usr/include/gtk-3.0,/usr/include,/usr/local/include,/usr/include/glib-2.0
export LVGL_INCLUDE=$LVGL_INCLUDE,/usr/lib/x86_64-linux-gnu/glib-2.0/include,/usr/include/pango-1.0
export LVGL_INCLUDE=$LVGL_INCLUDE,/usr/include/harfbuzz,/usr/include/cairo,/usr/include/gdk-pixbuf-2.0,/usr/include/atk-1.0,/usr/include/freetype2
export LVGL_LINK=gtk-3,gdk-3,pangocairo-1.0,pango-1.0,harfbuzz,atk-1.0,cairo-gobject,cairo,gdk_pixbuf-2.0,gio-2.0,gobject-2.0,glib-2.0
export LIBCLANG_PATH=$(llvm-config --libdir)
cargo build --release 
cargo run
