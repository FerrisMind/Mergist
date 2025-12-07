#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Скрипт для проверки проекта на наличие синтаксиса Svelte 4.
Находит все паттерны, которые нужно заменить на синтаксис Svelte 5.

Использование:
    python scripts/check-svelte4-syntax.py                    # Проверка всего проекта
    python scripts/check-svelte4-syntax.py --only-src        # Проверка только src/
    python scripts/check-svelte4-syntax.py --exclude-low     # Исключить низкоприоритетные проблемы
    python scripts/check-svelte4-syntax.py --output report.json  # Сохранить отчет в JSON

Примеры:
    # Проверка только основной директории без низкоприоритетных проблем
    python scripts/check-svelte4-syntax.py --only-src --exclude-low
    
    # Полная проверка с сохранением отчета
    python scripts/check-svelte4-syntax.py --output svelte4-report.json
"""

import os
import re
from pathlib import Path
from typing import List, Dict, Tuple
from dataclasses import dataclass
from collections import defaultdict

@dataclass
class Issue:
    """Представляет найденную проблему с синтаксисом Svelte 4"""
    file_path: str
    line_number: int
    issue_type: str
    description: str
    code_snippet: str

class Svelte4SyntaxChecker:
    """Проверяет файлы .svelte на наличие синтаксиса Svelte 4"""
    
    def __init__(self, root_dir: str = ".", exclude_low: bool = False, only_src: bool = False):
        self.root_dir = Path(root_dir)
        self.issues: List[Issue] = []
        self.exclude_low = exclude_low
        self.only_src = only_src
        
        # Паттерны для поиска синтаксиса Svelte 4
        self.patterns = {
            "export_let": {
                "pattern": r"^\s*export\s+let\s+\w+",
                "description": "Использование 'export let' для props (в Svelte 5 используется 'let { prop } = $props()')",
                "severity": "high"
            },
            "createEventDispatcher": {
                "pattern": r"createEventDispatcher",
                "description": "Использование 'createEventDispatcher' (в Svelte 5 используется '$props()' с функциями)",
                "severity": "high"
            },
            "reactive_statement": {
                "pattern": r"^\s*\$:\s+",
                "description": "Использование реактивных операторов '$:' (в Svelte 5 используется '$derived' или '$effect')",
                "severity": "high"
            },
            "slot_let": {
                "pattern": r"let:\w+",
                "description": "Использование директивы 'let:' для slots (в Svelte 5 используются snippets)",
                "severity": "medium"
            },
            "old_slots": {
                "pattern": r"<slot\s",
                "description": "Использование тега '<slot>' (в Svelte 5 используются snippets)",
                "severity": "high"
            },
            "svelte_component": {
                "pattern": r"<svelte:component\s",
                "description": "Использование '<svelte:component>' (в Svelte 5 не требуется, можно использовать компонент напрямую)",
                "severity": "low"
            },
            "old_props_access": {
                "pattern": r"\$\$props(?!\s*=)",
                "description": "Использование '$$props' без runes (в Svelte 5 используется '$props()')",
                "severity": "medium"
            },
            "old_rest_props": {
                "pattern": r"\$\$restProps",
                "description": "Использование '$$restProps' (в Svelte 5 используется '$props()' с spread)",
                "severity": "medium"
            },
            "bind_this_old": {
                "pattern": r"bind:this\s*=\s*\{[^}]+\}",
                "description": "Проверьте использование 'bind:this' - в Svelte 5 работает, но рекомендуется проверить",
                "severity": "low"
            },
            "old_context": {
                "pattern": r"getContext|setContext",
                "description": "Использование 'getContext'/'setContext' (в Svelte 5 рекомендуется использовать runes)",
                "severity": "low"
            },
            "old_store_syntax": {
                "pattern": r"\$\w+\s*=",
                "description": "Проверьте использование старых store подписок '$store' (может быть совместимо, но рекомендуется runes)",
                "severity": "low"
            }
        }
    
    def find_svelte_files(self) -> List[Path]:
        """Находит все файлы .svelte в проекте"""
        svelte_files = []
        # Директории, которые нужно исключить из проверки
        excluded_dirs = {
            'node_modules', '.git', 'build', 'dist', 'target', '.svelte-kit',
            'example', 'examples', 'test', 'tests', '__tests__', '.next',
            'coverage', '.cache', 'tmp', 'temp'
        }
        
        # Если указан флаг only_src, проверяем только src/
        search_dir = self.root_dir / 'src' if self.only_src else self.root_dir
        
        for root, dirs, files in os.walk(search_dir):
            # Пропускаем исключенные директории
            dirs[:] = [d for d in dirs if d not in excluded_dirs]
            
            # Пропускаем, если текущая директория в списке исключений
            if any(excluded in Path(root).parts for excluded in excluded_dirs):
                continue
            
            for file in files:
                if file.endswith('.svelte'):
                    svelte_files.append(Path(root) / file)
        
        return svelte_files
    
    def check_file(self, file_path: Path) -> List[Issue]:
        """Проверяет один файл на наличие синтаксиса Svelte 4"""
        issues = []
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
        except Exception as e:
            print(f"Ошибка при чтении файла {file_path}: {e}")
            return issues
        
        # Проверяем каждый паттерн
        for issue_type, config in self.patterns.items():
            pattern = re.compile(config["pattern"], re.MULTILINE)
            
            for line_num, line in enumerate(lines, 1):
                matches = pattern.finditer(line)
                for match in matches:
                    # Исключаем комментарии
                    if self._is_in_comment(line, match.start()):
                        continue
                    
                    # Исключаем строковые литералы
                    if self._is_in_string_literal(line, match.start()):
                        continue
                    
                    # Для export let проверяем, что это не внутри функции
                    if issue_type == "export_let":
                        if self._is_inside_function(lines, line_num - 1):
                            continue
                    
                    # Для reactive statements проверяем контекст
                    if issue_type == "reactive_statement":
                        # Пропускаем, если это уже внутри $effect или $derived
                        if self._is_inside_rune(lines, line_num - 1):
                            continue
                    
                    # Для старых store подписок - более строгая проверка
                    if issue_type == "old_store_syntax":
                        # Пропускаем, если это $state, $derived, $effect и т.д.
                        if re.search(r'\$state|\$derived|\$effect|\$props|\$bindable', line):
                            continue
                        # Пропускаем, если это просто переменная с $ в начале (не store)
                        if not re.search(r'\$\w+\s*[=:]', line):
                            continue
                    
                    # Пропускаем низкоприоритетные проблемы, если указан флаг
                    if self.exclude_low and config["severity"] == "low":
                        continue
                    
                    issues.append(Issue(
                        file_path=str(file_path.relative_to(self.root_dir)),
                        line_number=line_num,
                        issue_type=issue_type,
                        description=config["description"],
                        code_snippet=line.strip()
                    ))
        
        return issues
    
    def _is_in_comment(self, line: str, pos: int) -> bool:
        """Проверяет, находится ли позиция внутри комментария"""
        # Проверяем однострочные комментарии //
        if '//' in line:
            comment_pos = line.find('//')
            if pos > comment_pos:
                return True
        
        # Проверяем многострочные комментарии /* */
        comment_start = line.rfind('/*', 0, pos)
        comment_end = line.find('*/', pos)
        if comment_start != -1 and (comment_end == -1 or comment_end > pos):
            return True
        
        return False
    
    def _is_in_string_literal(self, line: str, pos: int) -> bool:
        """Проверяет, находится ли позиция внутри строкового литерала"""
        # Простая проверка на кавычки
        single_quotes = line[:pos].count("'") - line[:pos].count("\\'")
        double_quotes = line[:pos].count('"') - line[:pos].count('\\"')
        backticks = line[:pos].count('`') - line[:pos].count('\\`')
        
        # Если нечетное количество кавычек до позиции, значит мы внутри строки
        return (single_quotes % 2 == 1) or (double_quotes % 2 == 1) or (backticks % 2 == 1)
    
    def _is_inside_function(self, lines: List[str], line_index: int) -> bool:
        """Проверяет, находится ли строка внутри функции"""
        # Простая эвристика: ищем открывающие фигурные скобки
        brace_count = 0
        for i in range(line_index, -1, -1):
            line = lines[i]
            # Считаем открывающие и закрывающие скобки
            brace_count += line.count('{') - line.count('}')
            # Если находим function, class, if, for, while и т.д., и есть открытые скобки
            if brace_count > 0 and re.search(r'\b(function|class|if|for|while|switch)\s*\(', line):
                return True
            if brace_count < 0:
                break
        return False
    
    def _is_inside_rune(self, lines: List[str], line_index: int) -> bool:
        """Проверяет, находится ли строка внутри rune ($effect, $derived и т.д.)"""
        for i in range(max(0, line_index - 10), line_index):
            if re.search(r'\$effect|\$derived|\$state', lines[i]):
                return True
        return False
    
    def run(self) -> Dict[str, any]:
        """Запускает проверку всех файлов"""
        try:
            print("🔍 Поиск файлов .svelte...")
        except UnicodeEncodeError:
            print("[*] Поиск файлов .svelte...")
        svelte_files = self.find_svelte_files()
        try:
            print(f"📁 Найдено {len(svelte_files)} файлов .svelte\n")
            print("🔎 Проверка файлов на синтаксис Svelte 4...\n")
        except UnicodeEncodeError:
            print(f"[*] Найдено {len(svelte_files)} файлов .svelte\n")
            print("[*] Проверка файлов на синтаксис Svelte 4...\n")
        
        for file_path in svelte_files:
            file_issues = self.check_file(file_path)
            self.issues.extend(file_issues)
        
        return self.generate_report()
    
    def generate_report(self) -> Dict[str, any]:
        """Генерирует отчет о найденных проблемах"""
        # Группируем проблемы по типу
        issues_by_type = defaultdict(list)
        issues_by_file = defaultdict(list)
        
        for issue in self.issues:
            issues_by_type[issue.issue_type].append(issue)
            issues_by_file[issue.file_path].append(issue)
        
        # Статистика
        total_issues = len(self.issues)
        files_with_issues = len(issues_by_file)
        
        # Определяем эмодзи с обработкой ошибок кодировки
        emoji_ok = "✅"
        emoji_high = "🔴"
        emoji_medium = "🟡"
        emoji_low = "🟢"
        emoji_file = "📄"
        emoji_report = "📊"
        emoji_tip = "💡"
        emoji_save = "💾"
        
        # Пробуем вывести с эмодзи, если не получается - используем ASCII
        try:
            test_output = f"{emoji_report} test"
            print(test_output, end='')
            print('\r', end='')  # Возвращаем курсор
        except UnicodeEncodeError:
            emoji_ok = "[OK]"
            emoji_high = "[HIGH]"
            emoji_medium = "[MEDIUM]"
            emoji_low = "[LOW]"
            emoji_file = "[FILE]"
            emoji_report = "[REPORT]"
            emoji_tip = "[TIP]"
            emoji_save = "[SAVE]"
        
        print("=" * 80)
        print(f"{emoji_report} ОТЧЕТ О ПРОВЕРКЕ СИНТАКСИСА SVELTE 4")
        print("=" * 80)
        print(f"\nВсего найдено проблем: {total_issues}")
        print(f"Файлов с проблемами: {files_with_issues}")
        print(f"Всего проверено файлов: {len(self.find_svelte_files())}\n")
        
        if total_issues == 0:
            print(f"{emoji_ok} Отлично! Синтаксис Svelte 4 не найден. Проект использует только Svelte 5!")
            return {
                "total_issues": 0,
                "files_with_issues": 0,
                "issues_by_type": {},
                "issues_by_file": {}
            }
        
        # Выводим проблемы по типам
        print("ПРОБЛЕМЫ ПО ТИПАМ:")
        print("-" * 80)
        for issue_type, type_issues in sorted(issues_by_type.items(), key=lambda x: len(x[1]), reverse=True):
            severity = self.patterns[issue_type]["severity"]
            severity_emoji = {"high": emoji_high, "medium": emoji_medium, "low": emoji_low}.get(severity, "[?]")
            print(f"\n{severity_emoji} {issue_type.upper()} ({len(type_issues)} проблем, {severity} приоритет)")
            print(f"   {self.patterns[issue_type]['description']}")
        
        # Выводим проблемы по файлам
        print("\n\nПРОБЛЕМЫ ПО ФАЙЛАМ:")
        print("-" * 80)
        for file_path, file_issues in sorted(issues_by_file.items()):
            print(f"\n{emoji_file} {file_path} ({len(file_issues)} проблем)")
            for issue in file_issues:
                severity = self.patterns[issue.issue_type]["severity"]
                severity_emoji = {"high": emoji_high, "medium": emoji_medium, "low": emoji_low}.get(severity, "[?]")
                print(f"   {severity_emoji} Строка {issue.line_number}: {issue.description}")
                print(f"      {issue.code_snippet[:100]}...")
        
        print("\n" + "=" * 80)
        print(f"{emoji_tip} РЕКОМЕНДАЦИИ:")
        print("-" * 80)
        print(f"1. Используйте 'npx sv migrate svelte-5' для автоматической миграции")
        print(f"2. Высокоприоритетные проблемы ({emoji_high}) требуют немедленного исправления")
        print(f"3. Среднеприоритетные проблемы ({emoji_medium}) рекомендуется исправить")
        print(f"4. Низкоприоритетные проблемы ({emoji_low}) можно исправить позже")
        print("=" * 80)
        
        return {
            "total_issues": total_issues,
            "files_with_issues": files_with_issues,
            "issues_by_type": {k: len(v) for k, v in issues_by_type.items()},
            "issues_by_file": {k: len(v) for k, v in issues_by_file.items()}
        }


def main():
    """Главная функция"""
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Проверяет проект на наличие синтаксиса Svelte 4"
    )
    parser.add_argument(
        "--dir",
        type=str,
        default=".",
        help="Корневая директория проекта (по умолчанию: текущая директория)"
    )
    parser.add_argument(
        "--output",
        type=str,
        help="Путь к файлу для сохранения отчета в JSON формате"
    )
    parser.add_argument(
        "--exclude-low",
        action="store_true",
        help="Исключить низкоприоритетные проблемы из отчета"
    )
    parser.add_argument(
        "--only-src",
        action="store_true",
        help="Проверять только директорию src/"
    )
    
    args = parser.parse_args()
    
    checker = Svelte4SyntaxChecker(
        args.dir,
        exclude_low=args.exclude_low,
        only_src=args.only_src
    )
    report = checker.run()
    
    if args.output:
        import json
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(report, f, ensure_ascii=False, indent=2)
        try:
            print(f"\n💾 Отчет сохранен в {args.output}")
        except UnicodeEncodeError:
            print(f"\n[SAVE] Отчет сохранен в {args.output}")
    
    # Возвращаем код выхода в зависимости от наличия проблем
    exit_code = 1 if report["total_issues"] > 0 else 0
    exit(exit_code)


if __name__ == "__main__":
    main()

