#!/usr/bin/env python3
from __future__ import annotations

import argparse
from collections import deque
from pathlib import Path
import shutil
import subprocess

from PIL import Image


def remove_connected_light_background(image: Image.Image) -> Image.Image:
    rgba = image.convert("RGBA")
    pixels = rgba.load()
    width, height = rgba.size
    queue: deque[tuple[int, int]] = deque([(0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1)])
    visited: set[tuple[int, int]] = set()

    while queue:
        x, y = queue.popleft()
        if (x, y) in visited:
            continue
        visited.add((x, y))
        r, g, b, _ = pixels[x, y]
        if min(r, g, b) < 238 or max(r, g, b) - min(r, g, b) > 12:
            continue
        pixels[x, y] = (r, g, b, 0)
        if x > 0:
            queue.append((x - 1, y))
        if x + 1 < width:
            queue.append((x + 1, y))
        if y > 0:
            queue.append((x, y - 1))
        if y + 1 < height:
            queue.append((x, y + 1))

    return rgba


def save_resized(source: Image.Image, size: int, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    source.resize((size, size), Image.Resampling.LANCZOS).save(path, optimize=True)


def main() -> None:
    parser = argparse.ArgumentParser(description="Build Windows and macOS icons from the accepted source image.")
    parser.add_argument("source", type=Path)
    parser.add_argument("--output", type=Path, default=Path("apps/desktop/src-tauri/icons"))
    args = parser.parse_args()

    source = remove_connected_light_background(Image.open(args.source))
    output = args.output
    output.mkdir(parents=True, exist_ok=True)

    source.save(output / "icon.png", optimize=True)
    save_resized(source, 32, output / "32x32.png")
    save_resized(source, 128, output / "128x128.png")
    save_resized(source, 256, output / "128x128@2x.png")

    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    source.save(output / "icon.ico", sizes=ico_sizes)

    iconset = output / "VibeCodingRemote.iconset"
    shutil.rmtree(iconset, ignore_errors=True)
    iconset.mkdir()
    for points in (16, 32, 128, 256, 512):
        save_resized(source, points, iconset / f"icon_{points}x{points}.png")
        save_resized(source, points * 2, iconset / f"icon_{points}x{points}@2x.png")
    subprocess.run(["iconutil", "-c", "icns", str(iconset), "-o", str(output / "icon.icns")], check=True)
    shutil.rmtree(iconset)

    print(output / "icon.png")
    print(output / "icon.icns")
    print(output / "icon.ico")


if __name__ == "__main__":
    main()
