pub enum Fullscreen {
    Borderless,
    None
}
pub struct WindowSettings {
    pub title: &'static str,
    pub initial_size: [u32;2],
    pub max_size: Option<[u32;2]>,
    pub min_size: Option<[u32;2]>,
    pub position: [u32;2],
    pub fullscreen: Fullscreen,
    pub maximized: bool,
    pub resizable: bool,
    pub always_on_top: bool,
    pub decorations: bool
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "NDX",
            initial_size: [800, 600],
            max_size: None,
            min_size: None,
            position: [0, 0],
            fullscreen: Fullscreen::Borderless,
            maximized: true,
            resizable: false,
            always_on_top: false,
            decorations: false
        }
    }
}

pub struct CameraSettings {
    pub position: [f32;3],
    pub target: [f32;3],
    pub fov: f32,
    pub near: f32,
    pub far: f32
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 5.0],
            target: [0.0, 0.0, 0.0],
            fov: 80.0,
            near: 0.1,
            far: 100.0
        }
    }
}

pub struct Settings {
    pub window: WindowSettings,
    pub camera: CameraSettings,
    pub backend: wgpu::Backends,
    pub power_preference: wgpu::PowerPreference,
    pub present_mode: wgpu::PresentMode,
    /// forces wgpu to pick an adapter that will work on all hardware.
    /// This usually means that the rendering backend will use a "software" system, instead of hardware such as a GPU.
    pub force_fallback_adapter: bool
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            window: Default::default(),
            camera: Default::default(),
            backend: wgpu::Backends::all(),
            power_preference: wgpu::PowerPreference::HighPerformance,
            present_mode: wgpu::PresentMode::Fifo,
            force_fallback_adapter: false
        }
    }
}