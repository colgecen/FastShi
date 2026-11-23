#!/usr/bin/env python3
"""Capture the real FastShi window and wrap it in a macOS-style SVG frame.

Run from the repository root:

    python3 scripts/make_screenshots.py

The generated file is docs/img/app.svg. The GUI is rendered by the real
application; this script only captures the window, removes the desktop title
bar and draws the macOS window decoration around the captured content.
"""

from __future__ import annotations

import base64
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BINARY = ROOT / "target" / "debug" / "typing_trainer"
OUTPUT = ROOT / "docs" / "img" / "app.svg"

TITLE_BAR_HEIGHT = 40
TITLE_BAR_CROP = 30  # height of the KDE title bar in the captured window
CAPTURE_DELAY = 4.0


def run(cmd: list[str], **kwargs) -> None:
    subprocess.run(cmd, check=True, **kwargs)


def focus_app_window() -> None:
    """Ask KWin to focus the FastShi window when its scripting API is present."""
    if not shutil.which("gdbus"):
        return

    script = ROOT / "build" / ".cache" / "focus-fastshi.js"
    script.parent.mkdir(parents=True, exist_ok=True)
    script.write_text(
        "const wins = workspace.windowList();\n"
        "for (const w of wins) {\n"
        "    if (w.caption.indexOf('FastShi') !== -1) {\n"
        "        workspace.activeWindow = w;\n"
        "        break;\n"
        "    }\n"
        "}\n",
        encoding="utf-8",
    )

    plugin = f"fastshi-shot-{os.getpid()}"
    try:
        run(
            [
                "gdbus",
                "call",
                "--session",
                "--dest",
                "org.kde.KWin",
                "--object-path",
                "/Scripting",
                "--method",
                "org.kde.kwin.Scripting.loadScript",
                str(script),
                plugin,
            ],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        run(
            [
                "gdbus",
                "call",
                "--session",
                "--dest",
                "org.kde.KWin",
                "--object-path",
                "/Scripting",
                "--method",
                "org.kde.kwin.Scripting.start",
            ],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except (OSError, subprocess.CalledProcessError):
        # Other desktop environments simply keep the application focused.
        pass


def capture_active_window(destination: Path) -> None:
    if shutil.which("spectacle"):
        run(["spectacle", "-a", "-b", "-n", "-o", str(destination)])
    elif shutil.which("gnome-screenshot"):
        run(["gnome-screenshot", "-w", "-f", str(destination)])
    elif shutil.which("grim"):
        run(["grim", str(destination)])
    elif shutil.which("import"):
        run(["import", "-window", "root", str(destination)])
    else:
        raise SystemExit("No screenshot tool found (spectacle, gnome-screenshot, grim or import).")

    # Spectacle writes the file asynchronously through its D-Bus service.
    for _ in range(50):
        if destination.exists() and destination.stat().st_size > 0:
            return
        time.sleep(0.1)
    raise SystemExit("The screenshot tool did not write an image.")


def magick() -> str:
    return "magick" if shutil.which("magick") else "convert"


def prepare_content(source: Path, destination: Path) -> tuple[int, int]:
    """Remove the desktop title bar and scale the captured window content."""
    tool = magick()
    run(
        [
            tool,
            str(source),
            "-gravity",
            "North",
            "-chop",
            f"0x{TITLE_BAR_CROP}",
            "-resize",
            "1440x",
            "-strip",
            str(destination),
        ]
    )
    dimensions = subprocess.check_output(
        [tool, "identify", "-format", "%w %h", str(destination)]
    ).decode()
    width, height = (int(part) for part in dimensions.split())
    return width, height


def svg_frame(content: Path, content_width: int, content_height: int) -> str:
    encoded = base64.b64encode(content.read_bytes()).decode("ascii")
    total_height = content_height + TITLE_BAR_HEIGHT
    display_width = 800
    display_height = round(display_width * total_height / content_width)
    center = content_width / 2

    return f"""<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{display_width}" height="{display_height}" viewBox="0 0 {content_width} {total_height}">
  <defs>
    <clipPath id="window">
      <rect x="0" y="0" width="{content_width}" height="{total_height}" rx="12"/>
    </clipPath>
  </defs>
  <g clip-path="url(#window)">
    <rect x="0" y="0" width="{content_width}" height="{total_height}" fill="#1e1e1e"/>
    <rect x="0" y="0" width="{content_width}" height="{TITLE_BAR_HEIGHT}" fill="#2b2b2b"/>
    <image x="0" y="{TITLE_BAR_HEIGHT}" width="{content_width}" height="{content_height}" xlink:href="data:image/png;base64,{encoded}"/>
    <circle cx="24" cy="20" r="7" fill="#ff5f57"/>
    <circle cx="48" cy="20" r="7" fill="#febc2e"/>
    <circle cx="72" cy="20" r="7" fill="#28c840"/>
    <text x="{center}" y="26" text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif" font-size="16" fill="#e6e6e6">FastShi</text>
  </g>
  <rect x="0.5" y="0.5" width="{content_width - 1}" height="{total_height - 1}" rx="12" fill="none" stroke="#444444" stroke-width="1"/>
</svg>
"""


def main() -> None:
    run(["cargo", "build", "--quiet"], cwd=ROOT)

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)

    app = subprocess.Popen(
        [str(BINARY)],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        time.sleep(CAPTURE_DELAY)
        focus_app_window()
        time.sleep(1.0)
        with tempfile.TemporaryDirectory(prefix="fastshi-shot-") as temp_dir:
            captured = Path(temp_dir) / "window.png"
            content = Path(temp_dir) / "content.png"
            capture_active_window(captured)
            width, height = prepare_content(captured, content)
            OUTPUT.write_text(svg_frame(content, width, height), encoding="utf-8")
    finally:
        app.terminate()
        try:
            app.wait(timeout=5)
        except subprocess.TimeoutExpired:
            app.kill()
            app.wait()

    print(f"Wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    sys.exit(main())
