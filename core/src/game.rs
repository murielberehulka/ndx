use std::rc::Rc;
use futures::executor::block_on;

use crate::*;

pub struct Game {
    pub window: Window,
    pub device: Rc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub surface: Surface,
    pub depth_texture: texture::depth::Texture,
    pub close_requested: bool,
    pub lock_cursor: bool,
    pub camera: Camera,
    pub shaders: Shaders,
    pub animations: Vec<Animation>,
    pub diffuse_textures: Vec<texture::diffuse::Texture>,
    pub bgls: Rc<Bgls>,
    pub scene: Scene
}

impl Game {
    pub fn new(
        settings: Settings,
        event_loop: &winit::event_loop::EventLoop<()>
    ) -> Self {
        let window = Window::new(&settings.window, event_loop);
        let instance = wgpu::Instance::new(settings.backend);
        let (adapter, surface_format) = block_on(adapter::new(&settings, &instance, &window.window));
        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None
            },
            None
        )).unwrap();

        let surface = block_on(Surface::new(&settings, &window, &device, &instance, surface_format));
        let depth_texture = texture::depth::Texture::new(&device, &surface.config);
        let bgls = Rc::new(Bgls::new(&device));
        let camera = Camera::new(&settings.camera, &device, &window.size, &bgls);
        let shaders = crate::Shaders::new(&device, surface_format, &bgls);
        let animations = vec![];
        let diffuse_textures = vec![];
        let device = Rc::new(device);
        let scene = Scene::new(device.clone(), bgls.clone());

        Self {
            window,
            device: device.clone(),
            queue,
            surface,
            depth_texture,
            close_requested: false,
            lock_cursor: false,
            camera,
            shaders,
            animations,
            diffuse_textures,
            bgls: bgls.clone(),
            scene
        }
    }
    pub fn update(&mut self, control_flow: &mut winit::event_loop::ControlFlow) {
        self.camera.update(&self.queue);
        for mesh in &mut *self.scene.meshes {
            for model in &mut mesh.models {
                model.instances.update(&self.device);
            }
        }
        match self.render() {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost) => self.resize(self.window.size),
            Err(wgpu::SurfaceError::OutOfMemory) => {
                eprintln!("Out of memory error !");
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            Err(e) => eprintln!("Error getting current surface texture: {}", e)
        };
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output_texture = self.surface.surface.get_current_texture()?;
        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {label: None});
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0
                        }),
                        store: true
                    }
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true
                    }),
                    stencil_ops: None
                })
            });
            self.shaders.render(
                &mut render_pass,
                &self.camera.bind_group,
                &self.scene.meshes,
                &self.diffuse_textures
            );
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();
        Ok(())
    }
    pub fn resize(&mut self, mut new_window_size: winit::dpi::PhysicalSize<u32>) {
        if new_window_size.width == 0 {new_window_size.width = 1}
        if new_window_size.height == 0 {new_window_size.height = 1}
        if new_window_size.width < self.window.min_size[0] {new_window_size.width = self.window.min_size[0]}
        if new_window_size.height < self.window.min_size[1] {new_window_size.height = self.window.min_size[1]}
        if new_window_size.width > self.window.max_size[0] {new_window_size.width = self.window.max_size[0]}
        if new_window_size.height > self.window.max_size[1] {new_window_size.height = self.window.max_size[1]}
        self.surface.config.width = new_window_size.width;
        self.surface.config.height = new_window_size.height;
        self.surface.surface.configure(&self.device, &self.surface.config);
        self.depth_texture = texture::depth::Texture::new(&self.device, &self.surface.config);
        self.window.size = new_window_size;
        self.window.size_d2 = [self.window.size.width/2, self.window.size.height/2];
        self.camera.set_aspect((self.window.size.width / self.window.size.height)as f32);
    }
    pub fn set_focus(&mut self, focus: bool) {
        self.window.on_focus = focus;
        let active = self.lock_cursor && self.window.on_focus;
        self.window.window.set_cursor_grab(active).unwrap();
        self.window.window.set_cursor_visible(!active);
    }
    pub fn close(&mut self) {
        self.close_requested = true;
    }
    pub fn load_diffuse_texture<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.diffuse_textures.push(
            texture::diffuse::Texture::from_path(&self.device, &self.queue, &self.bgls.texture_diffuse, path.as_ref())
        )
    }
    pub fn load_diffuse_textures<P: AsRef<std::path::Path>>(&mut self, paths: &[P]) {
        for path in paths {
            self.load_diffuse_texture(path)
        }
    }
}