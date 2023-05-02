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
use lvgl::{Align, Color, DrawBuffer, Part, Widget};
use lvgl_sys::LV_LAYOUT_GRID;
use std::boxed::Box;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() -> LvResult<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f)} {l} {t} - {m}\n",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
    info!("Starting up");
    const HOR_RES: u32 = 1024;
    const VER_RES: u32 = 768;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();
    let display = lv_drv_disp_gtk!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let _input = lv_drv_input_pointer_gtk!(display)?;

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();

    unsafe {
        let x_grid = CoordDesc::<6>::from_values([
            100,
            100,
            100,
            100,
            100,
            lvgl_sys::LV_GRID_TEMPLATE_LAST.try_into().unwrap(),
        ]);
        let y_grid = CoordDesc::<6>::from_values([
            100,
            100,
            100,
            100,
            100,
            lvgl_sys::LV_GRID_TEMPLATE_LAST.try_into().unwrap(),
        ]);
        screen_style.set_grid_column_dsc_array(x_grid);
        screen_style.set_grid_row_dsc_array(y_grid);
    }

    screen.add_style(Part::Main, &mut screen_style)?;
    screen_style.set_layout(Layout::grid());

    // Create the button
    let mut button = Btn::create(&mut screen)?;
    let mut button_style = Style::default();
    button_style.set_grid_cell_column_pos(1);
    button_style.set_grid_cell_row_pos(1);
    button.add_style(Part::Main, &mut button_style)?;

    button.set_align(Align::LeftMid, 30, 0)?;
    button.set_size(100, 35)?;
    let mut btn_lbl = Label::create(&mut button)?;
    btn_lbl.set_text(CString::new("Click me!").unwrap().as_c_str())?;

    let mut btn_state = false;
    button.on_event(|_btn, event| {
        info!("Button received event: {:?}", event);
        if let lvgl::Event::Clicked = event {
            if btn_state {
                let nt = CString::new("Click me!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            } else {
                let nt = CString::new("Clicked!").unwrap();
                btn_lbl.set_text(nt.as_c_str()).unwrap();
            }
            btn_state = !btn_state;
        }
    })?;

    // Create the Label
    let mut label = Label::create(&mut screen)?;
    let mut label_style = Style::default();
    label_style.set_grid_cell_column_pos(2);
    label_style.set_grid_cell_row_pos(1);
    label_style.set_text_color(Color::from_rgb((255, 255, 255)));
    label.add_style(Part::Main, &mut screen_style)?;
    label.set_text(CString::new("Hello World!").unwrap().as_c_str())?;
    label.set_align(Align::Center, 0, 0)?;
    label.set_size(180, 80)?;

    // create the switch
    let mut switch = Btn::create(&mut screen)?;
    let mut switch_style = Style::default();
    switch_style.set_grid_cell_column_pos(1);
    switch_style.set_grid_cell_row_pos(3);
    switch_style.set_bg_color(Color::from_rgb((0, 0, 0)));

    switch.add_style(Part::Main, &mut switch_style)?;

    // Create the Meter

    let mut meter = Meter::create(&mut screen)?;

    let mut _arc = Arc::create(&mut meter)?;
    _arc.set_align(Align::Center, 0, 0)?;
    _arc.set_angles(260, 280)?;
    let mut _scale = Label::create(&mut meter)?;
    _scale.set_text(CString::new("Scale").unwrap().as_c_str())?;
    _scale.set_align(Align::Center, 0, 0)?;
    _scale.set_size(180, 180)?;
    _scale.set_pos(0, 0)?;

    meter.set_align(Align::RightMid, 30, 0)?;
    meter.set_size(180, 180)?;

    let mut meter_style: Style = Style::default();
    meter_style.set_bg_color(Color::from_rgb((0, 0, 0)));
    meter_style.set_bg_opa(Opacity::OPA_COVER);
    meter_style.set_line_color(Color::from_rgb((255, 255, 255)));
    meter_style.set_line_width(10);
    meter_style.set_line_opa(Opacity::OPA_70);
    meter_style.set_bg_grad_color(Color::from_rgb((255, 255, 255)));
    meter_style.set_arc_color(Color::from_rgb((255, 255, 255)));
    meter_style.set_grid_cell_column_pos(1);
    meter_style.set_grid_cell_row_pos(2);

    meter.add_style(Part::Main, &mut meter_style)?;

    loop {
        let start = Instant::now();
        lvgl::task_handler();
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }
}
