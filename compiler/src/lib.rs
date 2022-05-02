pub mod mesh;
pub use mesh::Mesh;
pub mod vertices;
pub use vertices::{Vertices, VertexType};

pub struct Assets {
    pub meshes: Vec<mesh::Mesh>
}

impl Assets {
    pub fn load(device: &wgpu::Device) -> Self {
        println!("Loading assets ...");
        let time = std::time::Instant::now();
        let b = match std::fs::read("./.assets/compiled.bin") {
            Ok(v) => v,
            Err(e) => panic!("Can not load './assets/compiled.bin, error: {}", e)
        };
        let mut i: usize = 0;
        let mut meshes = vec![];
        loop {
            match (b[i], b[i+1], b[i+2], b[i+3]) {
                // MESH
                (109, 101, 115, 104) => 
                    meshes.push(mesh::load(device, &b, &mut i)),
                // END
                (101, 110, 100, 95) => break,
                _ => panic!("Asset type '{}' ({:?}) not supported",
                        String::from_utf16_lossy(&[b[i]as u16, b[i+1]as u16, b[i+2]as u16, b[i+3]as u16]),
                        &b[i..i+4])
            }
        }
        println!("Assets loaded in: {} ms", time.elapsed().as_millis());
        Self {
            meshes
        }
    }
}