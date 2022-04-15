# NDX

![APIs](https://img.shields.io/badge/Rust-gray?logo=rust&style=flat-square)
![APIs](https://img.shields.io/badge/Vulkan-gray?logo=Vulkan&style=flat-square)
![Platforms](https://img.shields.io/badge/platforms-windows%20%7C%20linux%20%7C%20mac%20%7C%20android-red?style=flat-square)

NDX is a 3D game engine writed in rust.

## 🎯 Goal

Currently there is only a [Core](https://github.com/murielberehulka/ndx/blob/master/core/) for the engine, but i'm looking forward to make an GUI for manage projects and scene objects.

Also an compiler will be made, to compile all the meshes and textures into binary files.

## 🔎 Info

- The game is programmed using scripts.
- Every mesh has a vector of models an each model has multiple instances an the instances can have a skin if the mesh has a skeleton.
- To load a mesh you need to explicitly pass a VertexType, that says to the engine what properties to save, like vertex normal, uv, color, joints id and joints weights.

## 🚀 Running examples
```
cargo run --example <example-name>
```

choose one of the examples:
- [basic](https://github.com/murielberehulka/ndx/blob/master/core/examples/basic.rs)

## License
Use the code as you want :)