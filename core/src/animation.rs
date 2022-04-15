pub struct Keyframe {
    pub target: String,
    pub property: gltf::animation::Property,
    pub time: [f32;2],
    pub value: [[f32;3];2]
}

pub struct Animation {
    pub keyframes: Vec<Keyframe>
}

impl Animation {
}