"""
glb_reader.py — Pure-Python GLB/GLTF stats extraction.

Reads GLB binary format directly (no Blender, no bpy, no external deps).
GLB format: 12-byte header + JSON chunk + optional BIN chunk.
"""
from __future__ import annotations


import struct
import json
import os


def read_glb_stats(path: str) -> dict | None:
    """
    Extract mesh statistics from a GLB file without Blender.

    Returns a dict with:
      meshes, primitives, vertices, faces, materials,
      textures, has_embedded_textures, has_armature,
      animations, file_size_kb

    Returns None if file is not a valid GLB.
    """
    try:
        file_size = os.path.getsize(path)
        with open(path, "rb") as f:
            header = f.read(12)
            if len(header) < 12:
                return None
            magic, version, length = struct.unpack("<III", header)
            if magic != 0x46546C67:  # 'glTF'
                return None

            # Read JSON chunk
            chunk_header = f.read(8)
            if len(chunk_header) < 8:
                return None
            chunk_length, chunk_type = struct.unpack("<II", chunk_header)
            if chunk_type != 0x4E4F534A:  # 'JSON'
                return None
            raw_json = f.read(chunk_length)

        gltf = json.loads(raw_json.decode("utf-8").rstrip("\x00"))
    except Exception:
        return None

    meshes = gltf.get("meshes", [])
    accessors = gltf.get("accessors", [])
    materials = gltf.get("materials", [])
    images = gltf.get("images", [])
    skins = gltf.get("skins", [])
    animations = gltf.get("animations", [])

    total_prims = 0
    total_verts = 0
    total_faces = 0

    for mesh in meshes:
        for prim in mesh.get("primitives", []):
            total_prims += 1
            attrs = prim.get("attributes", {})
            # Vertex count from POSITION accessor
            if "POSITION" in attrs:
                acc_idx = attrs["POSITION"]
                if acc_idx < len(accessors):
                    total_verts += accessors[acc_idx].get("count", 0)
            # Face count from indices accessor (each 3 indices = 1 triangle)
            if "indices" in prim:
                acc_idx = prim["indices"]
                if acc_idx < len(accessors):
                    total_faces += accessors[acc_idx].get("count", 0) // 3

    # Embedded textures: images with a bufferView (binary data inside GLB)
    has_embedded = any(img.get("bufferView") is not None for img in images)

    # Extensions used (e.g. KHR_draco_mesh_compression, KHR_materials_unlit)
    extensions = list(gltf.get("extensionsUsed", []))

    return {
        "meshes": len(meshes),
        "primitives": total_prims,
        "vertices": total_verts,
        "faces": total_faces,
        "materials": len(materials),
        "textures": len(images),
        "has_embedded_textures": has_embedded,
        "has_armature": len(skins) > 0,
        "animations": len(animations),
        "extensions": extensions,
        "file_size_kb": file_size // 1024,
    }


def is_valid_glb(path: str) -> bool:
    """Quick check — does this file start with the GLB magic bytes?"""
    try:
        with open(path, "rb") as f:
            return f.read(4) == b"glTF"
    except Exception:
        return False
