pub async fn new(
    s: &crate::Settings,
    instance: &wgpu::Instance,
    window: &winit::window::Window
) -> (wgpu::Adapter, wgpu::TextureFormat) {
    let surface = unsafe { instance.create_surface(window) };
    instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: s.power_preference,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false
    }).await.unwrap();
    let mut format = wgpu::TextureFormat::Rgba32Float;
    let adapter = instance.enumerate_adapters(s.backend)
        .filter(|adapter| {
            // Check if this adapter supports our surface
            println!("Adapter: {:?}", &adapter);
            match surface.get_preferred_format(&adapter) {
                Some(v) => {
                    format = v;
                    true
                },
                None => false
            }
        })
        .next()
        .expect("Can't find an suitable adapter that supports the current surface !");
    (adapter, format)
}