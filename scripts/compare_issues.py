#!/usr/bin/env python3
"""
Утилита для сравнения двух файлов экспорта issues в Markdown.

Пример запуска (из папки tauri-app):
    python compare_issues.py heyputer-kv.js-issues.md heyputer-kv.js-issues-2025-12-07T02-57-50.md --limit 8 --sample 3
"""

from __future__ import annotations

import argparse
import sys
from collections import Counter
from pathlib import Path
from difflib import SequenceMatcher


def read_lines(path: Path) -> list[str]:
    try:
        return path.read_text(encoding="utf-8").splitlines()
    except FileNotFoundError:
        sys.exit(f"Файл не найден: {path}")


def main() -> None:
    parser = argparse.ArgumentParser(description="Сравнение двух Markdown-файлов с issues.")
    parser.add_argument("ours", help="Файл, сгенерированный приложением.")
    parser.add_argument("ref", help="Референсный файл.")
    parser.add_argument(
        "--limit",
        type=int,
        default=8,
        help="Сколько отличающихся хунков показать (по умолчанию 8).",
    )
    parser.add_argument(
        "--sample",
        type=int,
        default=3,
        help="Сколько строк примера выводить внутри хунка (по умолчанию 3).",
    )
    args = parser.parse_args()

    a_path = Path(args.ours)
    b_path = Path(args.ref)

    a = read_lines(a_path)
    b = read_lines(b_path)

    print(f"lines ours: {len(a)}")
    print(f"lines ref : {len(b)}")
    print(f"delta     : {len(a) - len(b)}")

    sm = SequenceMatcher(None, a, b, autojunk=False)
    ops = sm.get_opcodes()

    print("\nFirst differing hunks:")
    shown = 0
    for tag, i1, i2, j1, j2 in ops:
        if tag == "equal":
            continue
        shown += 1
        print(f"[{shown}] {tag} a[{i1}:{i2}] b[{j1}:{j2}] (len a={i2 - i1}, b={j2 - j1})")
        a_sample = "\n".join(a[i1 : min(i2, i1 + args.sample)])
        b_sample = "\n".join(b[j1 : min(j2, j1 + args.sample)])
        if a_sample:
            print("  ours:", a_sample.replace("\n", "\\n"))
        if b_sample:
            print("  ref :", b_sample.replace("\n", "\\n"))
        if shown >= args.limit:
            break

    cnt = Counter(tag for tag, *_ in ops)
    print("\nOpcode counts:", dict(cnt))


if __name__ == "__main__":
    main()

