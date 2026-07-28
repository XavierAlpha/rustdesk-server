import tempfile
import unittest
from pathlib import Path

from release_metadata import MetadataError, load_metadata


def write_metadata(root: Path, *, manifest: str, locked: str, debian: str) -> None:
    (root / "Cargo.toml").write_text(
        f'[package]\nname = "hbbs"\nversion = "{manifest}"\n',
        encoding="utf-8",
    )
    (root / "Cargo.lock").write_text(
        "version = 4\n\n"
        "[[package]]\n"
        'name = "hbbs"\n'
        f'version = "{locked}"\n',
        encoding="utf-8",
    )
    (root / "debian").mkdir()
    (root / "debian" / "changelog").write_text(
        f"rustdesk-server ({debian}) unstable; urgency=medium\n",
        encoding="utf-8",
    )


class ReleaseMetadataTests(unittest.TestCase):
    def test_accepts_consistent_stable_version(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_metadata(root, manifest="1.2.3", locked="1.2.3", debian="1.2.3")

            metadata = load_metadata(root)

            self.assertEqual(metadata.version, "1.2.3")
            self.assertEqual(metadata.tag, "v1.2.3")
            self.assertEqual((metadata.major, metadata.minor, metadata.patch), ("1", "2", "3"))

    def test_rejects_mismatched_or_ambiguous_versions(self) -> None:
        cases = (
            ("1.2.3", "1.2.2", "1.2.3"),
            ("1.2.3", "1.2.3", "1.2.3-1"),
            ("01.2.3", "01.2.3", "01.2.3"),
            ("1.2.3-rc.1", "1.2.3-rc.1", "1.2.3-rc.1"),
        )
        for manifest, locked, debian in cases:
            with self.subTest(manifest=manifest, locked=locked, debian=debian):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    write_metadata(root, manifest=manifest, locked=locked, debian=debian)
                    with self.assertRaises(MetadataError):
                        load_metadata(root)


if __name__ == "__main__":
    unittest.main()
