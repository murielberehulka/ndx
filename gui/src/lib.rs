mod style;
pub use style::*;
mod font;
pub use font::{Font, Fonts};

pub mod rect;
pub use rect::Rect;
pub mod text;
pub use text::Text;

pub enum Element {
    Rect(Rect),
    Text(Text)
}
impl Element {
    pub fn style(&self) -> &style::Style {
        match self {
            Element::Rect(s) => &s.style,
            Element::Text(s) => &s.style
        }
    }
    pub fn style_mut(&mut self) -> &mut style::Style {
        match self {
            Element::Rect(s) => &mut s.style,
            Element::Text(s) => &mut s.style
        }
    }
}

pub trait ElementTrait {
    fn element(self) -> Element;
}

pub struct Gui {
    elements: Vec<Element>,
    needs_update: bool,
    fonts: Fonts,
    rect: rect::Shader,
    text: text::Shader
}
impl Gui {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_diffuse_bgl: &wgpu::BindGroupLayout,
        texture_format: wgpu::TextureFormat,
        window_width: u32,
        window_height: u32
    ) -> Self {
        Self {
            elements: vec![Element::Rect(Rect {
                style: Style {
                    width: window_width,
                    height: window_height,
                    ..Default::default()
                }
            })],
            needs_update: true,
            fonts: Fonts::new(device, queue, texture_diffuse_bgl),
            rect: rect::Shader::new(device, texture_format),
            text: text::Shader::new(device, texture_diffuse_bgl, texture_format)
        }
    }
    pub fn update(&mut self, device: &wgpu::Device) {
        if !self.needs_update {return}
        self.needs_update = false;
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.elements[0].style_mut().width = w;
        self.elements[0].style_mut().height = h;
        self.needs_update = true;
    }
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.rect.render(render_pass);
        self.text.render(render_pass, &self.fonts);
    }
    pub fn add<E: ElementTrait>(&mut self, element: E) {
        self.elements.push(element.element());
        self.needs_update = true;
    }
}