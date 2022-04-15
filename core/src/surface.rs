pub struct Surface {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration
}

impl Surface {
    pub async fn new(
        s: &crate::Settings,
        window: &crate::Window,
        device: &wgpu::Device,
        instance: &wgpu::Instance,
        format: wgpu::TextureFormat
    ) -> Self {
        let surface = unsafe { instance.create_surface(&window.window) };
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: s.power_preference,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: window.size.width,
            height: window.size.height,
            present_mode: s.present_mode
        };
        surface.configure(device, &config);
        Self {
            surface,
            config
        }
    }
}