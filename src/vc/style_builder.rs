use std::vec::Vec;
use std::sync::Mutex;
use lvgl::style::{CoordDesc, GridAlign, Layout, Opacity, Style};
use lvgl::{Align, Animation, Color, Display, DrawBuffer, Event, Obj, Part, Widget};

static styles : Mutex<Vec<Style> >= Mutex::new(Vec::new());


pub struct StyleBuilder {
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
    fn build<'a>(&mut self) -> &'a mut Style {
        styles.push(self.style.clone());
        styles.last_mut().unwrap()
    }
}