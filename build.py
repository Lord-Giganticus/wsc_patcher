import platform
import subprocess
import os
from pathlib import Path
from zipfile import ZipFile, ZIP_LZMA

SYSTEM = platform.system().lower()

ZIP = Path(f"wsc_patcher_{SYSTEM}.zip").absolute()

subprocess.run("cargo build --release", shell=True)

with ZipFile(ZIP, "w", ZIP_LZMA) as w:
    os.chdir("target")
    subprocess.run("ls")
    os.chdir("release")
    if SYSTEM != "Windows":
        w.write("wsc_patcher", "wsc_patcher")
    else:
        w.write("wsc_patcher.exe", "wsc_patcher.exe")