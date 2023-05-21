use crate::vc::message::Sink;
use crate::Message;
use chrono::{DateTime, Local};
use core::fmt::Formatter;
use cstr_core::CString;
use log::info;
use std::{
    fmt::{Debug, Error},
    ops::ShrAssign,
};
use yaml_rust::Yaml;

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

pub struct VcLabel {
    topic: String,
    value: String,
    prefix: String,
    suffix: String,
    time: DateTime<Local>,
    lv_label: Label,
}

impl VcLabel {
    pub fn new(cont: &mut Obj, config: Yaml, channel: pub_sub::PubSub<Message>) -> VcLabel {
        info!("Creating label : {:?}", config);
        let mut label = VcLabel {
            topic: config["src"].as_str().unwrap_or("NoTopic").to_string(),
            value: config["value"].as_str().unwrap_or("NoValue").to_string(),
            prefix: config["prefix"].as_str().unwrap_or("").to_string(),
            suffix: config["suffix"].as_str().unwrap_or("").to_string(),
            time: Local::now(),
            lv_label: Label::create(cont).unwrap(),
        };
        let text = format!("{}{}{}", label.prefix, label.value, label.suffix);
        match label.lv_label.set_text(&CString::new(text).unwrap()) {
            Ok(_) => {}
            Err(e) => {
                info!("Error setting label text: {:?}", e);
            }
        }
        let mut style = Style::default();
        style.set_grid_cell_column_pos(config["x"].as_i64().unwrap_or(200) as i16);
        style.set_grid_cell_row_pos(config["y"].as_i64().unwrap_or(200) as i16);
        style.set_grid_cell_column_span(config["w"].as_i64().unwrap_or(1) as i16);
        style.set_grid_cell_row_span(config["h"].as_i64().unwrap_or(1) as i16);
        match label.lv_label.add_style(Part::Main, &mut style) {
            Ok(_) => {}
            Err(e) => {
                info!("Error setting label style: {:?}", e);
            }
        }
        label
    }
}

impl Sink<Message> for VcLabel {
    fn on(&mut self, t: &Message) -> () {
        match t {
            Message::Publish { topic, value, time } => {
                info!("compare topic: {:?} {:?} ", &topic, &self.topic);
                if topic == &self.topic {
                    self.value = value.to_string();
                    self.time = time.clone();
                    let text = format!("{}{}{}", self.prefix, self.value, self.suffix);
                    match self.lv_label.set_text(&CString::new(text).unwrap()) {
                        Ok(_) => {}
                        Err(e) => {
                            info!("Error setting label text: {:?}", e);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
