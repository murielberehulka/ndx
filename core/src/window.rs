use winit::{
    window::WindowBuilder,
    dpi::PhysicalPosition
};

pub struct Window {
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub size_d2: [u32;2],
    pub position: [u32;2],
    pub on_focus: bool,
    pub min_size: [u32;2],
    pub max_size: [u32;2]
}

impl Window {
    pub fn new(s: &crate::WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let ms = {
            let temp_event_loop = winit::event_loop::EventLoop::new();
            let temp_window = match WindowBuilder::new()
                .with_title(s.title)
                .with_visible(false)
                .build(&temp_event_loop) {Ok(v) => v,Err(e) => panic!("Error creating window: {}", e)};
            temp_window.current_monitor().unwrap().size()
        };
        let inner_size = winit::dpi::PhysicalSize {width: s.initial_size[0], height: s.initial_size[1]};
        let min_size = match s.min_size {
            Some(v) => winit::dpi::PhysicalSize {width: v[0], height: v[1]},
            None => winit::dpi::PhysicalSize {width: 1, height: 1}
        };
        let max_size = match s.max_size {
            Some(v) => winit::dpi::PhysicalSize {width: v[0], height: v[1]},
            None => winit::dpi::PhysicalSize {width: ms.width, height: ms.height}
        };
        let position = PhysicalPosition::new(s.position[0], s.position[1]);
        let window = match WindowBuilder::new()
            .with_position(position)
            .with_fullscreen(
                match s.fullscreen {
                    crate::Fullscreen::Borderless => Some(winit::window::Fullscreen::Borderless(None)),
                    crate::Fullscreen::None => None
                }
            )
            .with_inner_size(inner_size)
            .with_min_inner_size(min_size)
            .with_max_inner_size(max_size)
            .with_maximized(s.maximized)
            .with_resizable(s.resizable)
            .with_decorations(s.decorations)
            .with_title(s.title)
            .with_always_on_top(s.always_on_top)
            .build(event_loop) {
                Ok(v) => v,
                Err(e) => panic!("Error creating window: {}", e)
            };
        window.focus_window();
        Self {
            window,
            size: inner_size,
            size_d2: [inner_size.width/2,inner_size.height/2],
            position: [position.x, position.y],
            on_focus: true,
            min_size: [min_size.width, min_size.height],
            max_size: [max_size.width, max_size.height]
        }
    }
}