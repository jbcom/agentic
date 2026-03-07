"""
render_preview.py — Headless Blender script to render a GLB as a 512x512 PNG.

Usage (called as subprocess from server.py):
    blender --background --python render_preview.py -- --input model.glb --output preview.png

Camera: isometric-ish 3/4 view. Lighting: 3-point studio. Output: 512x512 RGBA PNG.
"""
from __future__ import annotations


import sys
import os
import argparse
import math

try:
    import bpy  # noqa: F401
    HAS_BPY = True
except ImportError:
    HAS_BPY = False


def parse_args():
    if "--" in sys.argv:
        args = sys.argv[sys.argv.index("--") + 1:]
    else:
        args = []
    p = argparse.ArgumentParser()
    p.add_argument("--input", required=True)
    p.add_argument("--output", required=True)
    p.add_argument("--size", type=int, default=512)
    return p.parse_args(args)


def main():
    import bpy
    import mathutils

    args = parse_args()

    # Clear scene
    bpy.ops.wm.read_factory_settings(use_empty=True)

    # Import GLB
    bpy.ops.import_scene.gltf(filepath=os.path.abspath(args.input))

    # Find imported mesh objects and compute bounding box
    meshes = [o for o in bpy.context.scene.objects if o.type == "MESH"]
    if not meshes:
        print(f"[preview] ERROR: no mesh in {args.input}")
        sys.exit(1)

    # Compute world bounding box center and radius
    all_corners = []
    for obj in meshes:
        for corner in obj.bound_box:
            all_corners.append(obj.matrix_world @ mathutils.Vector(corner))

    xs = [v.x for v in all_corners]
    ys = [v.y for v in all_corners]
    zs = [v.z for v in all_corners]
    center = mathutils.Vector((
        (min(xs) + max(xs)) / 2,
        (min(ys) + max(ys)) / 2,
        (min(zs) + max(zs)) / 2,
    ))
    radius = max(
        max(xs) - min(xs),
        max(ys) - min(ys),
        max(zs) - min(zs),
    ) / 2 or 1.0

    # Camera: 3/4 isometric view
    cam_data = bpy.data.cameras.new("PreviewCam")
    cam_data.type = "PERSP"
    cam_data.lens = 50
    cam_obj = bpy.data.objects.new("PreviewCam", cam_data)
    bpy.context.scene.collection.objects.link(cam_obj)
    bpy.context.scene.camera = cam_obj

    distance = radius * 3.5
    angle_h = math.radians(45)
    angle_v = math.radians(30)
    cam_obj.location = center + mathutils.Vector((
        distance * math.cos(angle_v) * math.cos(angle_h),
        distance * math.cos(angle_v) * math.sin(angle_h),
        distance * math.sin(angle_v),
    ))
    direction = center - cam_obj.location
    rot_quat = direction.to_track_quat("-Z", "Y")
    cam_obj.rotation_euler = rot_quat.to_euler()

    # 3-point lighting
    def add_light(name, light_type, energy, loc):
        ld = bpy.data.lights.new(name, light_type)
        ld.energy = energy
        lo = bpy.data.objects.new(name, ld)
        bpy.context.scene.collection.objects.link(lo)
        lo.location = center + mathutils.Vector(loc) * radius * 3

    add_light("Key", "SUN", 3.0, (1, -1, 2))
    add_light("Fill", "SUN", 1.0, (-2, 1, 1))
    add_light("Back", "SUN", 0.5, (0, 2, -0.5))

    # World background: neutral grey
    bpy.context.scene.world = bpy.data.worlds.new("World")
    bpy.context.scene.world.use_nodes = True
    bg = bpy.context.scene.world.node_tree.nodes["Background"]
    bg.inputs["Color"].default_value = (0.15, 0.15, 0.15, 1.0)
    bg.inputs["Strength"].default_value = 0.3

    # Render settings
    scene = bpy.context.scene
    scene.render.engine = "CYCLES"
    scene.cycles.samples = 32
    scene.cycles.use_denoising = True
    scene.render.resolution_x = args.size
    scene.render.resolution_y = args.size
    scene.render.film_transparent = False
    scene.render.image_settings.file_format = "PNG"
    scene.render.filepath = os.path.abspath(args.output)

    bpy.ops.render.render(write_still=True)
    print(f"[preview] Rendered: {args.output}")


if __name__ == "__main__":
    main()
