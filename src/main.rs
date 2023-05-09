// Still WIP
//#![allow(unused_labels)]
//#![allow(unused_variables)]
//#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(dead_code)]

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
use lvgl::style::{CoordDesc, GridAlign, Layout, Opacity, Style};
use lvgl::widgets::MeterPart::Needle;
use lvgl::widgets::{Arc, Bar, Btn, Img, Label, Meter, MeterPart, Slider, Switch, Table, Textarea};
use lvgl::Align::*;
use lvgl::LvResult;
use lvgl::{Align, Animation, Color, DrawBuffer, Event, Obj, Part, Widget};
use lvgl_sys::LV_LAYOUT_GRID;
use lvgl_sys::_LV_FLEX_COLUMN;
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
    const COL_COUNT: u32 = 24;
    const ROW_COUNT: u32 = 24;
    const COL_WIDTH: u32 = HOR_RES / COL_COUNT;
    const ROW_HEIGHT: u32 = VER_RES / ROW_COUNT;
    const SQUARE_FACTOR: f64 = HOR_RES as f64 / VER_RES as f64;

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
    let x_array = [COL_WIDTH as i16; COL_COUNT as usize];
    let y_array = [ROW_HEIGHT as i16; ROW_COUNT as usize];

    unsafe {
        const XSIZE: usize = COL_COUNT as usize;
        const YSIZE: usize = ROW_COUNT as usize;
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

    //let mut buttons = Vec::new();
    let mut styles = Vec::new();

    {
        let mut _btn = Btn::create(&mut cont)?;
        let mut _btn_style = new_grid_style(&mut styles, 0, 0, 2, 1);
        let mut _btn_label = Label::create(&mut _btn)?;
        let cstr = CString::new("Button").unwrap();
        _btn_label.set_text(&cstr)?;
        _btn_label.set_align(Align::Center, 0, 0)?;
        _btn.add_style(Part::Main, &mut _btn_style)?;
        _btn.on_event(|_btn, event| match event {
            Event::Clicked => {
                info!("Button clicked");
            }
            _ => {}
        })?;
    }
    {
        let mut switch = Switch::create(&mut cont)?;
        let switch_style = new_grid_style(&mut styles, 0, 1, 1, 1);
        switch.add_style(Part::Main, switch_style)?;
    }

    let mut bar = Bar::create(&mut cont)?;
    let bar_style = new_grid_style(&mut styles, 0, 2, 3, 1);
    bar.add_style(Part::Main, bar_style)?;
    bar.set_value(50, Animation::OFF)?;

    let mut meter = Meter::create(&mut cont)?;
    let meter_style = new_grid_style(&mut styles, 0, 3, 3, (3.0 * SQUARE_FACTOR) as i16);
    meter.add_style(Part::Main, meter_style)?;

    {
        // slider box
        let mut cont_slider = Obj::create(&mut cont)?;
        let cont_slider_style = new_grid_style(&mut styles, 0, 7, 4, 1);
        cont_slider_style.set_layout(Layout::flex());
        cont_slider_style.set_flex_flow(_LV_FLEX_COLUMN);
        cont_slider_style.set_flex_grow(1);
        cont_slider_style.set_height(2 * ROW_HEIGHT as i16);
        cont_slider_style.set_width(3 * COL_WIDTH as i16);
        cont_slider_style.set_radius(0);
        cont_slider_style.set_border_side(1);
        cont_slider.add_style(Part::Main, cont_slider_style)?;

        let mut slider = Slider::create(&mut cont_slider)?;
        let slider_style = new_style(&mut styles);
        slider.add_style(Part::Main, slider_style)?;
        slider.on_event(|_slider, _event| match _event {
            Event::Released => {
                let val = _slider.get_value();
                info!("Slider value released : {}", val.unwrap());
            }
            Event::ValueChanged => {
                let val = _slider.get_value();
                info!("Slider value changed : {}", val.unwrap());
                bar.set_value(val.unwrap(), Animation::ON).unwrap();
            }
            _ => {}
        })?;

        let mut slider_label = Label::create(&mut cont_slider)?;
        let str1 = CString::new("Label").unwrap();
        slider_label.set_text(&str1)?;
    }

    {
        let mut table = Table::create(&mut cont)?;
        let table_style = new_grid_style(&mut styles, 7, 2, 8,8);
        table.add_style(Part::Main, table_style)?;
        table.set_col_cnt(3)?;
        table.set_row_cnt(3)?;
        /*     table.set_col_width(0, 100)?;
        table.set_col_width(1, 100)?;
        table.set_col_width(2, 100)?;
        table.set_row_height(0, 50)?;
        table.set_row_height(1, 50)?;
        table.set_row_height(2, 50)?;*/
        table.set_cell_value(0, 0, &mut CString::new("1").unwrap())?;
        table.set_cell_value(0, 1, &CString::new("2").unwrap())?;
        table.set_cell_value(0, 2, &CString::new("3").unwrap())?;
        table.set_cell_value(1, 0, &CString::new("4").unwrap())?;
        table.set_cell_value(1, 1, &CString::new("5").unwrap())?;
        table.set_cell_value(1, 2, &CString::new("6").unwrap())?;
        table.set_cell_value(2, 0, &CString::new("7").unwrap())?;
        table.set_cell_value(2, 1, &CString::new("8").unwrap())?;
        table.set_cell_value(2, 2, &CString::new("9").unwrap())?;
        table.on_event(|_table, _event| match _event {
            _ => {
                info!("Table event : {:?}", _event);
            }
        })?;
    }
    /*{
        let mut image = Img::create(&mut cont)?;
        let image_style = new_grid_style(&mut styles, 1, 4, 1, 3);
        image.add_style(Part::Main, image_style)?;
    }*/
    /*    for i in 0..(COL_COUNT * ROW_COUNT) {
        let col = i % COL_COUNT;
        let row = i / COL_COUNT;

        let mut btn = {
            //            buttons.push(Btn::create(&mut cont)?);
            //           buttons.last_mut().unwrap()
            Btn::create(&mut cont)?
        };
        let btn_style = {
            styles.push(Style::default());
            styles.last_mut().unwrap()
        };
        if row == col {
            btn_style.set_bg_color(Color::from_rgb((0, 0, 255)));
        } else {
            btn_style.set_bg_color(Color::from_rgb((255, 255, 255)));
            btn_style.set_text_color(Color::from_rgb((0, 0, 0)));
        }
        btn_style.set_grid_cell_column_pos(col as i16);
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

        info!("i {} row {} col {}", i, row, col);
        btn.add_style(Part::Main, btn_style)?;

        let mut label = Label::create(&mut btn)?;
        let s = CString::new(format!("c{}, r{} ", col, row)).unwrap();
        label.set_text(&s)?;
        label.set_align(Align::Center, 0, 0)?;
    }*/
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
// to avoid the leakage of the styles
fn new_style<'a>(v: &'a mut Vec<Style>) -> &'a mut Style {
    let _style = StyleBuilder::new()
        .set_pad_bottom(0)
        .set_pad_top(0)
        .set_pad_left(0)
        .set_pad_right(0)
        .build();
    v.push(_style);

    let style = v.last_mut().unwrap();
    style
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
    fn build(&mut self) -> Style {
        self.style.clone()
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
