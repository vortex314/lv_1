// Still WIP
//#![allow(unused_labels)]
//#![allow(unused_variables)]
//#![allow(unreachable_code)]
#![allow(unused_imports)]

use cstr_core::CString;
use log::{info, warn, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use lvgl::input_device::InputDriver;
use lvgl::lv_drv_disp_gtk;
use lvgl::lv_drv_disp_sdl;
use lvgl::lv_drv_input_pointer_gtk;
use lvgl::lv_drv_input_pointer_sdl;
use lvgl::style::{CoordDesc, Layout, Opacity, Style,GridAlign};
use lvgl::widgets::MeterPart::Needle;
use lvgl::widgets::{Arc, Btn, Label, Meter, MeterPart};
use lvgl::LvResult;
use lvgl::{Align, Color, DrawBuffer, Obj, Part, Widget};
use lvgl_sys::LV_LAYOUT_GRID;
use lvgl::Align::*;
use std::boxed::Box;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use yaml_rust::YamlLoader;

fn main() -> LvResult<()> {
    set_logging();
    load_yaml();
    info!("Starting up");
    const HOR_RES: u32 = 1024;
    const VER_RES: u32 = 768;
    const COL_COUNT:u32=24;
    const ROW_COUNT:u32=24;
    const COL_WIDTH :u32= HOR_RES / COL_COUNT;
    const ROW_HEIGHT :u32= VER_RES / ROW_COUNT;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES / 10) as usize }>::default();
 //   let display = lv_drv_disp_sdl!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
 //   let _input = lv_drv_input_pointer_sdl!(display)?;
    let display = lv_drv_disp_gtk!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let _input = lv_drv_input_pointer_gtk!(display)?;   
 //   let display = lv_drv_disp_fb!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
 //   let _input = lv_drv_input_pointer_fb!(display)?;   

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;
    let mut screen_style = Style::default();
    screen_style.set_pad_bottom(0);
    screen_style.set_pad_top(0);
    screen_style.set_pad_left(0);
    screen_style.set_pad_right(0);
    screen.add_style(Part::Main, &mut screen_style)?;

    let mut cont = Obj::create(&mut screen)?;

    let mut cont_style = Style::default();
    let  x_array = [COL_WIDTH as i16; COL_COUNT as usize];
    let  y_array = [ROW_HEIGHT as i16; ROW_COUNT as usize];

    unsafe {
        const XSIZE : usize = COL_COUNT as usize;
        const YSIZE : usize = ROW_COUNT as usize;
        let x_grid = CoordDesc::<XSIZE>::from_values(x_array, true);
        let y_grid = CoordDesc::<YSIZE>::from_values(y_array, true);
        cont_style.set_grid_column_dsc_array(&x_grid);
        cont_style.set_grid_row_dsc_array(&y_grid);
    }
    cont_style.set_pad_row(0);
    cont_style.set_pad_column(0);
    cont_style.set_pad_left(0);
    cont_style.set_pad_right(0);
    cont_style.set_pad_top(0);
    cont_style.set_pad_bottom(0);

    cont_style.set_width(HOR_RES as i16);
    cont_style.set_height(VER_RES as i16);
    cont_style.set_align(Align::Center);
    cont.add_style(Part::Main, &mut cont_style)?;
    cont_style.set_layout(Layout::grid());

    let mut buttons = Vec::new();
    let mut styles = Vec::new();

    for i in 0..(COL_COUNT * ROW_COUNT) {
        let col = i % COL_COUNT;
        let row = i / ROW_COUNT;

        let btn = {
            buttons.push(Btn::create(&mut cont)?);
            buttons.last_mut().unwrap()
        };
        let btn_style =
        {
            styles.push(Style::default());
            styles.last_mut().unwrap()
        };
        if  row == col {
            btn_style.set_bg_color(Color::from_rgb((0, 0, 255)));
        } else {
            btn_style.set_bg_color(Color::from_rgb((255, 0, 0)));
        }
        btn_style.set_grid_cell_column_pos(col as i16 );
        btn_style.set_grid_cell_row_pos(row as i16);
        btn_style.set_grid_cell_column_span(1);
        btn_style.set_grid_cell_row_span(1);
        btn_style.set_grid_cell_x_align(GridAlign::STRETCH);
        btn_style.set_grid_cell_y_align(GridAlign::STRETCH);
        btn_style.set_width(COL_WIDTH as i16);
        btn_style.set_height(ROW_HEIGHT as i16);
        btn_style.set_pad_top(0);
        btn_style.set_pad_bottom(0);
        btn_style.set_pad_left(0);
        btn_style.set_pad_right(0);
        btn_style.set_radius(0);

    
 //       info!(" row {} col {}", row, col);
        btn.add_style(Part::Main, btn_style)?;
    
        let mut label = Label::create(btn)?;
        let s = CString::new(format!("c{}, r{} ", col, row)).unwrap();
        label.set_text(&s)?;
        label.set_align(Align::Center, 0, 0)?;
    }
    info!("start loop");
    loop {
        let start = Instant::now();
        lvgl::task_handler();
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }
}

fn set_logging() {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {t} - {m}\n",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn load_yaml() -> Vec<yaml_rust::Yaml> {
    let source = "
person:
    - name: John Doe
      age: 43
      phones:
        - \"555-555-5555\"
        - \"666-666-6666\"
";
    let docs = YamlLoader::load_from_str(source).unwrap();
    info!("Yaml docs: {:?}", docs);

    docs
}
