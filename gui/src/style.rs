pub enum Align {
    LeftTop,
    CenterTop,
    RightTop,
    LeftBottom,
    CenterBottom,
    RightBottom,
    Center
}

pub struct Style {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub background: Background,
    pub align: Align,
    pub z_index: f32,
    pub parent: usize
}
impl Default for Style {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            x: 0,
            y: 0,
            background: Background::Solid(Color::Hex(0xff030303)),
            align: Align::LeftTop,
            z_index: 0.0,
            parent: 0
        }
    }
}
impl Style {
    pub fn position(&self, elements: &Vec<super::Element>) -> [i32;2] {
        let [px, py] = if self.parent == 0 { [0, 0] } else {
            elements[self.parent].style().position(elements)
        };
        let pw = elements[self.parent].style().width;
        let ph = elements[self.parent].style().height;
        match self.align {
            Align::LeftTop => [self.x + px, self.y + ph as i32],
            Align::CenterTop => [self.x + ((pw as i32 - self.width as i32)/2), self.y - ph as i32],
            Align::RightTop => [self.x + (pw as i32 - self.width as i32), self.y - ph as i32],
            Align::LeftBottom => [self.x + px, self.y + py],
            Align::CenterBottom => [self.x + ((pw as i32 - self.width as i32)/2), self.y],
            Align::RightBottom => [self.x + (pw as i32 - self.width as i32), self.y],
            Align::Center => [self.x + ((pw as i32 - self.width as i32)/2), self.y + ((ph as i32 - self.height as i32)/2)]
        }
    }
    pub fn position_center(&self, elements: &Vec<super::Element>) -> [f32;2] {
        let [x, y] = self.position(elements);
        [
            ((x as f32 / elements[0].style().width as f32) * 2.0) - 1.0,
            ((y as f32 / elements[0].style().height as f32) * 2.0) - 1.0
        ]
    }
}

pub enum Color {
    Raw (f32, f32, f32, f32),
    Rgb (u32, u32, u32),
    Rgba (u32, u32, u32, u32),
    Hex (u32)
}
impl Color {
    pub fn as_raw(&self) -> [f32;4] {
        match self {
            Color::Raw (r, g, b, a) => [*r, *g, *b, *a],
            Color::Rgb (r, g, b) => [*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, 1.0],
            Color::Rgba (r, g, b, a) => [*r as f32 / 255.0, *g as f32 / 255.0, *b as f32 / 255.0, *a as f32 / 255.0],
            Color::Hex (v) => [
                (*v >> 16) as u8 as f32 / 255.0,
                (*v >> 8) as u8 as f32 / 255.0,
                *v as u8 as f32 / 255.0,
                (*v >> 24) as u8 as f32 / 255.0,
            ]
        }
    }
}

pub enum Background {
    Solid (Color),
    // TODO
    /* LeftToRight ([f32;3], [f32;3]),
    TopToBottom ([f32;3], [f32;3]),
    LeftTopToRightBottom ([f32;3], [f32;3]),
    RightTopToLeftBottom ([f32;3], [f32;3]),
    Corners ([f32;3], [f32;3], [f32;3], [f32;3]) */
}