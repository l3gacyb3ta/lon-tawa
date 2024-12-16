use crate::{
    helper::{plot_pixel, HEIGHT, WIDTH},
    Drawable, Text,
};

#[derive(Clone)]
pub struct Button {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: u32,
    text_renderer: Text,
    text: String,
}

impl Button {
    fn new(x: f64, y: f64, color: u32, width: f64, height: f64, text: String) -> Button {
        Button {
            x,
            y,
            width,
            height,
            color,
            text_renderer: Text::new(WIDTH, HEIGHT, 1),
            text,
        }
    }
}

impl Drawable for Button {
    fn draw(&self, buffer: &mut [u32], _: f32, _: f32, _: (f64, f64)) {
        rectangle(buffer, self.x, self.y, self.width, self.height, self.color);
        self.text_renderer
            .draw(buffer, (self.x as usize + 2, self.y as usize - self.height as usize + 3), &self.text);
    }
}

pub struct UserInterface {
    buttons: Vec<Button>
}

pub fn dda_line(buffer: &mut [u32], x0: f64, y0: f64, x1: f64, y1: f64, color: u32) {
    let mut x0 = x0.floor() as i32;
    let mut y0 = y0.floor() as i32;
    let x1 = x1.floor() as i32;
    let y1 = y1.floor() as i32;

    let dx = if x1 > x0 { x1 - x0 } else { x0 - x1 };
    let dy = if y1 > y0 { y1 - y0 } else { y0 - y1 };

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = (if dx > dy { dx } else { -dy }) / 2;
    let mut e2;

    loop {
        plot_pixel(buffer, x0 as usize, y0 as usize, color);
        if x0 == x1 && y0 == y1 {
            break;
        }

        e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy
        }
    }
}

fn rectangle(buffer: &mut [u32], x: f64, y: f64, width: f64, height: f64, color: u32) {
    dda_line(buffer, x, y, x + width, y, color); // Top left to top right
    dda_line(buffer, x, y, x, y - height, color); // Top Left to Bottom Left
    dda_line(buffer, x, y - height, x + width, y - height, color); // Bottom Left to Bottom Right
    dda_line(buffer, x + width, y - height, x + width, y, color); // Bottom Right to Top Right
}

impl UserInterface {
    pub fn new() -> UserInterface {
        UserInterface {
            buttons: vec![
                Button::new(100., 100., 0xffff00ff, 100., 20., "Paused".to_owned())
            ],
        }
    }
}

impl Drawable for UserInterface {
    fn draw(&self, buffer: &mut [u32], _: f32, _: f32, _: (f64, f64)) {
        for button in self.buttons.clone().into_iter() {
            button.draw(buffer, 0., 0., (0., 0.));
        }
    }

    
}
