from pathlib import Path
import shutil
import fnmatch
import tomllib

ROOT = Path(__file__).parent
DIST = ROOT / "dist" / "crawssembly"

with open(ROOT / "release.toml", "rb") as f: config = tomllib.load(f)

includes = config["include"]
excludes = config["exclude"]

def ignored(path):
  rel = path.relative_to(ROOT).as_posix()
  return any(fnmatch.fnmatch(rel, pattern) for pattern in excludes)

def copy_item(src):
  rel = src.relative_to(ROOT)
  dst = DIST / rel

  if ignored(src): return

  if src.is_dir():
    for item in src.rglob("*"):
      if item.is_file() and not ignored(item):
        out = DIST / item.relative_to(ROOT)
        out.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(item, out)
  elif src.is_file():
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)

if DIST.exists(): shutil.rmtree(DIST)

DIST.mkdir(parents=True)

for item in includes: copy_item(ROOT / item)

print(f"Clean release written to {DIST}")
