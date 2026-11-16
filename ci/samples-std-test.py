#!/usr/bin/env python3
"""The samples job must give both pinned tools the same standard library."""
import pathlib
import subprocess
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
STAGER = ROOT / "ci" / "stage-samples-std.py"
COMMAND = (
    "python3 wolf-book/ci/stage-samples-std.py "
    "wolf-lang/target/release/std wolf-interp/target/release/std"
)


def files(root):
    return {p.relative_to(root).as_posix(): p.read_bytes()
            for p in root.rglob("*") if p.is_file()}


class SamplesStd(unittest.TestCase):
    def test_workflow_stages_std_before_running_samples(self):
        workflow = (ROOT / ".github" / "workflows" / "book.yml").read_text()
        self.assertIn(COMMAND, workflow)
        self.assertIn("python3 ci/samples-std-test.py", workflow)
        built = workflow.index("- name: build pinned lupin (release)")
        staged = workflow.index("- name: stage pinned std beside lupin")
        samples = workflow.index("- name: run every sample")
        self_test = workflow.index("- name: self-test (a deliberately-broken sample must fail)")
        self.assertLess(built, staged)
        self.assertLess(staged, samples)
        self.assertLess(samples, self_test)

    def test_stager_replaces_stale_tree_and_preserves_every_byte(self):
        self.assertTrue(STAGER.is_file(), "the std staging helper must exist")
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            source = root / "wolf-lang" / "target" / "release" / "std"
            dest = root / "wolf-interp" / "target" / "release" / "std"
            (source / "list").mkdir(parents=True)
            (source / "STD-REV").write_bytes(b"pinned-std\n")
            (source / "LICENSE").write_bytes(b"fixture license\n")
            (source / "list" / "list.lu").write_bytes(b"// UTF-8: \xe7\x8b\xbc\n")
            dest.mkdir(parents=True)
            (dest / "stale.lu").write_bytes(b"old cache entry\n")
            result = subprocess.run(
                [sys.executable, str(STAGER), str(source), str(dest)],
                capture_output=True, text=True,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(files(dest), files(source))
            self.assertFalse((dest / "std").exists(), "do not nest std/std on cache reuse")


if __name__ == "__main__":
    unittest.main()
