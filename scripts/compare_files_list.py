#!/usr/bin/env python3
"""
Сравнение списков файлов в двух Markdown-выгрузках (по строкам `// File: ...`).

Пример:
    python scripts/compare_files_list.py our.md ref.md
"""

from __future__ import annotations

import sys
from pathlib import Path


def extract_paths(md_path: Path) -> list[str]:
    lines = md_path.read_text(encoding="utf-8").splitlines()
    return [line[len("// File: ") :].strip() for line in lines if line.startswith("// File: ")]


def main() -> None:
    if len(sys.argv) != 3:
        sys.exit("Использование: python scripts/compare_files_list.py <ours.md> <ref.md>")

    ours = Path(sys.argv[1])
    ref = Path(sys.argv[2])

    if not ours.exists():
        sys.exit(f"Не найден файл: {ours}")
    if not ref.exists():
        sys.exit(f"Не найден файл: {ref}")

    a = extract_paths(ours)
    b = extract_paths(ref)

    extra = sorted(set(a) - set(b))
    missing = sorted(set(b) - set(a))

    print(f"files in ours   : {len(a)}")
    print(f"files in ref    : {len(b)}")
    print(f"extra (ours-ref): {len(extra)}")
    print(f"missing (ref-ours): {len(missing)}\n")

    if extra:
        print("Extra:")
        for p in extra:
            print("  ", p)
    if missing:
        print("Missing:")
        for p in missing:
            print("  ", p)


if __name__ == "__main__":
    main()


