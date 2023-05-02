
## References :
https://github.com/lvgl/lv_drivers/blob/master/gtkdrv/README.md

## Prelude install
```
   sudo apt-get install libgtk-3-dev
   sudo apt-get install libglib2.0-dev
```
## lv_drv_conf.h
```
#  define USE_GTK       1
```
## lv_conf.h
```
/*Color depth: 1 (1 byte per pixel), 8 (RGB332), 16 (RGB565), 32 (ARGB8888)*/
#define LV_COLOR_DEPTH 32
```
## In the source of your Application
```
    const HOR_RES: u32 = 300;
    const VER_RES: u32 = 300;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();
    let display = lv_drv_disp_gtk!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let _input = lv_drv_input_pointer_gtk!(display)?;
```


## Build it
```
export DEP_LV_CONFIG_PATH=`pwd`
export LVGL_INCLUDE=/usr/include/gtk-3.0,/usr/include,/usr/local/include,/usr/include/glib-2.0
export LVGL_INCLUDE=$LVGL_INCLUDE,/usr/lib/x86_64-linux-gnu/glib-2.0/include,/usr/include/pango-1.0
export LVGL_INCLUDE=$LVGL_INCLUDE,/usr/include/harfbuzz,/usr/include/cairo,/usr/include/gdk-pixbuf-2.0,/usr/include/atk-1.0,/usr/include/freetype2
export LVGL_LINK=gtk-3,gdk-3,pangocairo-1.0,pango-1.0,harfbuzz,atk-1.0,cairo-gobject,cairo,gdk_pixbuf-2.0,gio-2.0,gobject-2.0,glib-2.0
export LIBCLANG_PATH=$(llvm-config --libdir)
cargo build --release 
cargo run
```