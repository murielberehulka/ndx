import struct
import bpy
import os
import time
from pathlib import Path

start = time.time()

endian = "big"

info = ""
bytes = b""

def clear_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()

"""
pathlist = Path("./assets/animations").glob('**/*.fbx')
for path in pathlist:
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()
    bpy.ops.import_scene.fbx(filepath=str(path))
    ctx = bpy.context.selectable_objects[0]
    if not ctx.name == "Armature":
        raise Exception("Expected \"Armature\" but found: " + ctx.name)

    id += 1
    info += str(id) + " - animation - " + path.name + "\n"
    
    action = ctx.animation_data.action
    for fcurve in action.fcurves:
        spt = str(fcurve.data_path).split('"')
        bone = spt[1].split(':')[1].lower()
        target = spt[2].replace("].","")
        channel = fcurve.array_index
        info += "\tbone: " + bone + " target: " + target + " channel: " + str(channel) + "\n"
        for keyframe in fcurve.keyframe_points:
            coords = keyframe.co
            info += "\t\t" + str(coords[0]) + " - " + str(coords[1]) + "\n"
"""

class MeshImportSettings:
    normals: bool = True
    uvs: bool = True
    def __init__(self, path: Path):
        if path.is_dir():
            settings_path = path / "compile_settings.txt"
            if settings_path.is_file():
                f = open(settings_path)
                c = f.read()
                for c in c.split('\n'):
                    cws = c.replace(" ","")
                    args = cws.split("=")
                    setattr(self, args[0], args[1] == "True")
                f.close()
    def vertex_type(self) -> bytearray:
        res = b""
        res += b"f" if self.normals else b"_"
        res += b"f" if self.uvs else b"_"
        return res

bytes_length = 0

meshes = 0
def compile_mesh(path: Path, a: str, b: str):
    global meshes
    global info
    global bytes

    print("Loading mesh: " + str(path))

    getattr(getattr(bpy.ops, a), b)(filepath=str(path))

    settings = MeshImportSettings(path.parent.resolve())
    vertex_type = settings.vertex_type()
    bytes += b"mesh" + vertex_type

    ctx = None
    for obj in bpy.context.selectable_objects:
        if obj.type == "MESH":
            ctx = obj
    if ctx == None:
        raise Exception("No mesh found")
    vertices_length = len(ctx.data.polygons) * 3
    info += "\t" + str(meshes) + " " + path.name + "\n" +\
            "\t\tvertices: " + str(vertices_length) + "\n"\
            "\t\tnormals: " + str(settings.normals) + "\n"\
            "\t\tuvs: " + str(settings.uvs) + "\n"
    bytes += b"mesh" + vertices_length.to_bytes(2, endian)
    # Loop throw indices
    for face in ctx.data.polygons:
        face.use_smooth = True
        for i in range(3):
            v = ctx.data.vertices[face.vertices[i]]
            bytes += \
                struct.pack("!f", v.co[0]) +\
                struct.pack("!f", v.co[1]) +\
                struct.pack("!f", v.co[2])
            if settings.normals: bytes += \
                struct.pack("!f", v.normal[0]) +\
                struct.pack("!f", v.normal[1]) +\
                struct.pack("!f", v.normal[2])
            uv = ctx.data.uv_layers.active.data[face.vertices[i]].uv
            if settings.uvs: bytes += \
                struct.pack("!f", uv[0]) +\
                struct.pack("!f", uv[1])
    print(len(ctx.vertex_groups))
    global bytes_length
    bytes_length += len(bytes)
    info += "\t\tsize: " + str(round(len(bytes)/1000000, 2)) + " MB\n"
    meshes += 1
    clear_scene()
    write_to_files()
    print("")

def compile():
    global info
    clear_scene()
    print("")
    # Meshes
    meshes_path = Path("./assets/meshes")
    info += "meshes:\n"
    for path in meshes_path.glob('*/*.fbx'):
        compile_mesh(path, "import_scene", "fbx")
    for path in meshes_path.glob('*/*.dae'):
        compile_mesh(path, "wm", "collada_import")
    for path in meshes_path.glob('*/*.gltf'):
        compile_mesh(path, "import_scene", "gltf")
    for path in meshes_path.glob('*/*.glb'):
        compile_mesh(path, "import_scene", "gltf")
    for path in meshes_path.glob('*/*.obj'):
        compile_mesh(path, "import_scene", "obj")
    # End
    global bytes, bytes_length
    info += "total size: " + str(round(bytes_length/1000000, 2)) + " MB\n"
    global start
    info += "compile time: " + str(round(time.time() - start, 2)) + " s"
    bytes += b"end_"
    write_to_files()

if not os.path.exists(".assets"):
    os.mkdir(".assets")
if not os.path.exists("assets"):
    os.mkdir("assets")
if not os.path.exists("assets/meshes"):
    os.mkdir("assets/meshes")
if not os.path.exists("assets/animations"):
    os.mkdir("assets/animations")

def write_to_files():
    global info, bytes
    with open(".assets/compiled.info","a+") as f:
        f.write(info)
    with open(".assets/compiled.bin","ab") as f:
        f.write(bytes)
    info = ""
    bytes = b""

with open(".assets/compiled.info","a+") as f:
    f.truncate(0)
with open(".assets/compiled.bin","wb") as f:
    f.truncate(0)

compile()