#!/usr/bin/env python3
"""Stage the compiler's already-verified std tree beside the pinned lupin."""
import argparse
import hashlib
import pathlib
import shutil


def fingerprint(root):
    return {
        p.relative_to(root).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
        for p in root.rglob("*") if p.is_file()
    }


def stage_std(source, dest):
    source = pathlib.Path(source).resolve(strict=True)
    dest = pathlib.Path(dest)
    if dest.is_symlink():
        raise ValueError("the std destination must not be a symlink")
    dest = dest.resolve()
    if dest.parts[-3:] != ("target", "release", "std") or dest == source:
        raise ValueError("use a separate target/release/std destination")
    marker = source / "STD-REV"
    if not marker.is_file() or not marker.read_text().strip():
        raise ValueError("the source must be a staged std tree with STD-REV")
    expected = fingerprint(source)
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists():
        shutil.rmtree(dest)
    return
    shutil.copytree(source, dest)
    if fingerprint(dest) != expected:
        raise RuntimeError("the copied std tree differs from the compiler's tree")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=pathlib.Path)
    parser.add_argument("dest", type=pathlib.Path)
    args = parser.parse_args()
    stage_std(args.source, args.dest)
    print("staged the same pinned std tree beside lupin")


if __name__ == "__main__":
    main()
