// Still WIP
//#![allow(unused_labels)]
//#![allow(unused_variables)]
//#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::Message;
use chrono::{DateTime, Local};
use core::mem::MaybeUninit;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use cstr_core::CString;
// use gtk::glib::Date;
use log::{info, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use lvgl::input_device::pointer::Pointer;
use lvgl::input_device::InputDriver;
use lvgl::lv_drv_disp_gtk;
use lvgl::lv_drv_disp_sdl;
use lvgl::lv_drv_input_pointer_evdev;
use lvgl::lv_drv_input_pointer_gtk;
use lvgl::lv_drv_input_pointer_sdl;
use lvgl::style::{CoordDesc, GridAlign, Layout, Opacity, Style};
use lvgl::widgets::MeterPart::Needle;
use lvgl::widgets::{Arc, Bar, Btn, Img, Label, Meter, MeterPart, Slider, Switch, Table, Textarea};
use lvgl::Align::*;
use lvgl::LvResult;
use lvgl::{lv_drv_disp_fbdev, LvError};
use lvgl::{Align, Animation, Color, Display, DrawBuffer, Event, Obj, Part, Widget};
use lvgl_sys::lv_table_get_selected_cell;
use std::boxed::Box;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use yaml_rust::YamlLoader;

const HOR_RES: u32 = 1024;
const VER_RES: u32 = 768;

#[derive(Debug, Clone)]

enum Sorting {
    OnTopic,
    OnValue,
    OnCount,
    OnTime,
}

struct Entry {
    topic: String,
    value: String,
    time: DateTime<Local>,
    count: i64,
}

#[cfg(target_arch = "arm")]
fn display_init() -> LvResult<(Display, Pointer)> {
    info!("Initializing Raspberry Pi display");

    const COL_COUNT: u32 = 24;
    const ROW_COUNT: u32 = 24;
    const COL_WIDTH: u32 = HOR_RES / COL_COUNT;
    const ROW_HEIGHT: u32 = VER_RES / ROW_COUNT;
    const SQUARE_FACTOR: f64 = HOR_RES as f64 / VER_RES as f64;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES / 10) as usize }>::default();
    let display = lv_drv_disp_fbdev!(buffer, HOR_RES, VER_RES)?; // Use this for fb0 (Raspberry Pi)
    let _input = lv_drv_input_pointer_evdev!(display)?; // check which evdev /dev/input/eventXXX
    info!("Display initialized resolution {} x {}", HOR_RES, VER_RES);
    Ok((display, _input))
}

#[cfg(target_arch = "x86_64")]
fn display_init() -> LvResult<(Display, Pointer)> {
    info!("Initializing GTK display");

    const COL_COUNT: u32 = 24;
    const ROW_COUNT: u32 = 24;
    const COL_WIDTH: u32 = HOR_RES / COL_COUNT;
    const ROW_HEIGHT: u32 = VER_RES / ROW_COUNT;
    const SQUARE_FACTOR: f64 = HOR_RES as f64 / VER_RES as f64;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES / 10) as usize }>::default();
    let display = lv_drv_disp_gtk!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let input = lv_drv_input_pointer_gtk!(display)?;
    info!("Display initialized resolution {} x {}", HOR_RES, VER_RES);
    Ok((display, input))
}

#[cfg(target_arch = "aarch64")]
fn display_init() -> LvResult<(Display, Pointer)> {
    info!("Initializing SDL2 display");

    const COL_COUNT: u32 = 24;
    const ROW_COUNT: u32 = 24;
    const COL_WIDTH: u32 = HOR_RES / COL_COUNT;
    const ROW_HEIGHT: u32 = VER_RES / ROW_COUNT;
    const SQUARE_FACTOR: f64 = HOR_RES as f64 / VER_RES as f64;

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES / 10) as usize }>::default();
    let display = lv_drv_disp_sdl!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let input = lv_drv_input_pointer_sdl!(display)?;
    info!("Display initialized resolution {} x {}", HOR_RES, VER_RES);
    Ok((display, input))
}
/*
fn display_sdl_init(){
    let display = lv_drv_disp_sdl!(buffer, HOR_RES, VER_RES)?; // Use this for GTK (Linux)
    let _input = lv_drv_input_pointer_sdl!(display)?;
}
*/
pub fn do_view(send: Sender<Message>, recv: Receiver<Message>) -> LvResult<()> {
    let mut styles = Vec::<Style>::new();

    let (display, pointer) = display_init().unwrap();

    // Create screen and widgets
    let mut screen = display.get_scr_act()?;
    let mut screen_style = StyleBuilder::new()
        .set_pad_bottom(0)
        .set_pad_top(0)
        .set_pad_left(0)
        .set_pad_right(0)
        .set_width(HOR_RES as i16)
        .set_height(VER_RES as i16)
        .build(&mut styles);
    screen.add_style(Part::Main, &mut screen_style)?;

    let mut table = Table::create(&mut screen)?;

    let header_style = StyleBuilder::new()
        .set_bg_color(Color::from_rgb((0, 0, 0)))
        .set_text_color(Color::from_rgb((0, 255, 0)))
        .set_border_width(0)
        .set_pad_bottom(0)
        .set_pad_top(0)
        .set_pad_left(0)
        .set_pad_right(0)
        .build(&mut styles);
    table.add_style(Part::Items, header_style)?;

    let _table_style = StyleBuilder::new()
        .set_bg_color(Color::from_rgb((255, 255, 255)))
        .set_width(HOR_RES as i16)
        .set_height(VER_RES as i16)
        .set_pad_bottom(0)
        .set_pad_top(0)
        .set_pad_left(0)
        .set_pad_right(0)
        .build(&mut styles);
    table.add_style(Part::Main, _table_style)?;

    //   table.add_style(Part::Indicator, header_style)?;
    table.set_col_cnt(4)?;
    table.set_row_cnt(45)?;
    table.set_col_width(0, 300)?;
    table.set_col_width(1, 500)?;
    table.set_col_width(2, 100)?;
    table.set_col_width(3, 124)?;

    table.set_cell_value(0, 0, &mut CString::new("Topic").unwrap())?;
    table.set_cell_value(0, 1, &CString::new("Message").unwrap())?;
    table.set_cell_value(0, 2, &CString::new("Count").unwrap())?;
    table.set_cell_value(0, 3, &CString::new("Time").unwrap())?;

    let mut sorting = Sorting::OnTopic;
    table.on_event(|mut _table, _event| match _event {
        Event::ValueChanged => {
            let (row, col) = _table.get_selected_cell().unwrap();
            if row == 0 {
                match col {
                    0 => sorting = Sorting::OnTopic,
                    1 => sorting = Sorting::OnValue,
                    2 => sorting = Sorting::OnCount,
                    3 => sorting = Sorting::OnTime,
                    _ => {}
                }
                info!("Sorting on {:?}", sorting);
                send.send(Message::Refresh).unwrap();
            }
        }
        _ => {
            info!("Event {:?}", _event);
        }
    })?;

    let mut tab = HashMap::<String, Entry>::new();

    info!("start loop");
    loop {
        let start = Instant::now();
        lvgl::task_handler();
        let res = recv.recv_timeout(Duration::from_millis(15));
        match res {
            Ok(message) => {
                match message {
                    Message::Refresh => {
                        update_table_view(&mut table, &tab, sorting.clone()).unwrap();
                    }
                    Message::Publish { topic, value, time } => {
                        update_table(&mut tab, Message::Publish { topic, value, time });
                        update_table_view(&mut table, &tab, sorting.clone()).unwrap();
                    }
                    _ => {}
                };
            }
            Err(e) => {}
        }
        lvgl::tick_inc(Instant::now().duration_since(start));
    }
}

fn update_table(tab: &mut HashMap<String, Entry>, m: crate::Message) {
    match m {
        Message::Publish { topic, value, time } => {
            if tab.contains_key(&topic) {
                let mut entry = tab.get_mut(&topic).unwrap();
                entry.value = value.clone();
                entry.time = time;
                entry.count += 1;
            } else {
                let entry = Entry {
                    topic: topic.clone(),
                    value: value.clone(),
                    time: time,
                    count: 1,
                };
                tab.insert(topic.clone(), entry);
            }
        }
        _ => {}
    }
}

fn update_table_view(
    table: &mut Table,
    tab: &HashMap<String, Entry>,
    sorting: Sorting,
) -> Result<(), LvError> {
    let mut idx = 1;
    let mut sorted: Vec<_> = tab.iter().collect();
    match sorting {
        Sorting::OnTopic => {
            sorted.sort_by(|a, b| a.1.topic.cmp(&b.1.topic));
        }
        Sorting::OnValue => {
            sorted.sort_by(|a, b| a.1.value.cmp(&b.1.value));
        }
        Sorting::OnCount => {
            sorted.sort_by(|a, b| a.1.count.cmp(&b.1.count));
        }
        Sorting::OnTime => {
            sorted.sort_by(|a, b| a.1.time.cmp(&b.1.time));
        }
    }
    for entry in sorted.iter() {
        table.set_cell_value(idx, 0, &CString::new(entry.1.topic.clone()).unwrap())?;
        table.set_cell_value(idx, 1, &CString::new(entry.1.value.clone()).unwrap())?;
        table.set_cell_value(idx, 2, &CString::new(entry.1.count.to_string()).unwrap())?;
        table.set_cell_value(
            idx,
            3,
            &CString::new(entry.1.time.format("%H:%M:%S").to_string()).unwrap(),
        )?;
        idx += 1;
    }
    Ok(())
}

// to avoid the leakage of the styles
fn new_style<'a>(v: &'a mut Vec<Style>) -> &'a mut Style {
    StyleBuilder::new()
        .set_pad_bottom(0)
        .set_pad_top(0)
        .set_pad_left(0)
        .set_pad_right(0)
        .build(v)
}

fn new_grid_style<'a>(
    v: &'a mut Vec<Style>,
    x: i16,
    y: i16,
    x_size: i16,
    y_size: i16,
) -> &'a mut Style {
    let style = new_style(v);
    //   StyleBuilder::from(style);
    style.set_grid_cell_column_pos(x);
    style.set_grid_cell_row_pos(y);
    style.set_grid_cell_column_span(x_size);
    style.set_grid_cell_row_span(y_size);
    style.set_align(Align::Center);
    style.set_grid_cell_x_align(GridAlign::STRETCH);
    style.set_grid_cell_y_align(GridAlign::STRETCH);
    style
}

struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    fn new() -> StyleBuilder {
        StyleBuilder {
            style: Style::default(),
        }
    }
    fn from(style: Style) -> StyleBuilder {
        StyleBuilder { style }
    }
    fn set_grid_cell_column_pos(&mut self, x: i16) -> &mut StyleBuilder {
        self.style.set_grid_cell_column_pos(x);
        self
    }
    fn set_grid_cell_row_pos(&mut self, y: i16) -> &mut StyleBuilder {
        self.style.set_grid_cell_row_pos(y);
        self
    }
    fn set_grid_cell_column_span(&mut self, x_size: i16) -> &mut StyleBuilder {
        self.style.set_grid_cell_column_span(x_size);
        self
    }
    fn set_grid_cell_row_span(&mut self, y_size: i16) -> &mut StyleBuilder {
        self.style.set_grid_cell_row_span(y_size);
        self
    }
    fn set_align(&mut self, align: Align) -> &mut StyleBuilder {
        self.style.set_align(align);
        self
    }
    fn set_grid_cell_x_align(&mut self, align: GridAlign) -> &mut StyleBuilder {
        self.style.set_grid_cell_x_align(align);
        self
    }
    fn set_grid_cell_y_align(&mut self, align: GridAlign) -> &mut StyleBuilder {
        self.style.set_grid_cell_y_align(align);
        self
    }
    fn set_width(&mut self, width: i16) -> &mut StyleBuilder {
        self.style.set_width(width);
        self
    }
    fn set_height(&mut self, height: i16) -> &mut StyleBuilder {
        self.style.set_height(height);
        self
    }
    fn set_pad_top(&mut self, pad: i16) -> &mut StyleBuilder {
        self.style.set_pad_top(pad);
        self
    }
    fn set_pad_bottom(&mut self, pad: i16) -> &mut StyleBuilder {
        self.style.set_pad_bottom(pad);
        self
    }
    fn set_pad_left(&mut self, pad: i16) -> &mut StyleBuilder {
        self.style.set_pad_left(pad);
        self
    }
    fn set_pad_right(&mut self, pad: i16) -> &mut StyleBuilder {
        self.style.set_pad_right(pad);
        self
    }
    fn set_radius(&mut self, radius: i16) -> &mut StyleBuilder {
        self.style.set_radius(radius);
        self
    }
    fn set_bg_color(&mut self, color: Color) -> &mut StyleBuilder {
        self.style.set_bg_color(color);
        self
    }
    fn set_bg_opa(&mut self, opa: Opacity) -> &mut StyleBuilder {
        self.style.set_bg_opa(opa);
        self
    }
    fn set_bg_grad_color(&mut self, color: Color) -> &mut StyleBuilder {
        self.style.set_bg_grad_color(color);
        self
    }
    fn set_text_color(&mut self, color: Color) -> &mut StyleBuilder {
        self.style.set_text_color(color);
        self
    }
    fn set_border_width(&mut self, width: i16) -> &mut StyleBuilder {
        self.style.set_border_width(width);
        self
    }
    fn build<'a>(&mut self, styles: &'a mut Vec<Style>) -> &'a mut Style {
        styles.push(self.style.clone());
        styles.last_mut().unwrap()
    }
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
