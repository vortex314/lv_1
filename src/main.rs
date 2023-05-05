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
use lvgl::lv_drv_input_pointer_gtk;
use lvgl::style::{CoordDesc, Layout, Opacity, Style};
use lvgl::widgets::MeterPart::Needle;
use lvgl::widgets::{Arc, Btn, Label, Meter, MeterPart};
use lvgl::LvResult;
use lvgl::{Align, Color, DrawBuffer, Obj, Part, Widget};
use lvgl_sys::LV_LAYOUT_GRID;
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

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES / 10) as usize }>::default();
    let display = lv_drv_disp_gtk!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let _input = lv_drv_input_pointer_gtk!(display)?;

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;
    // cont.set_size(300, 200)?;

    let mut cont = Obj::create(&mut screen)?;

    let mut cont_style = Style::default();

    unsafe {
        let x_grid = CoordDesc::<3>::from_values([70, 70, 70], true);
        let y_grid = CoordDesc::<3>::from_values([50, 50, 50], true);
        cont_style.set_grid_column_dsc_array(&x_grid);
        cont_style.set_grid_row_dsc_array(&y_grid);
    }
    cont_style.set_align(Align::Center);
    cont_style.set_layout(Layout::grid());
    cont_style.set_width(300);
    cont_style.set_height(200);

    cont.add_style(Part::Main, &mut cont_style)?;

    let mut buttons = Vec::new();

    for i in 0..9 {
        let col = i % 3;
        let row = i / 3;

        let mut btn = Btn::create(&mut cont)?;
        let mut btn_style = Style::default();
        btn_style.set_grid_cell_column_pos(col);
        btn_style.set_grid_cell_row_pos(row);
        btn_style.set_grid_cell_column_span(2);
        btn_style.set_grid_cell_row_span(2);
        btn_style.set_width(70);
        btn_style.set_height(50);
        info!(" row {} col {}", row, col);
        btn.add_style(Part::Any, &mut btn_style)?;

        let mut label = Label::create(&mut btn)?;
        let s = CString::new(format!("c{}, r{} ", col, row)).unwrap();
        label.set_text(&s)?;
        label.set_align(Align::Center, 0, 0)?;
        buttons.push(btn);
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
