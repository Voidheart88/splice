#!/usr/bin/env python3
"""
Public API Analyzer for Splice

This script analyzes the Rust source code to identify all public items
and generates a comprehensive report of the public API surface.

Usage:
    python scripts/analyze_public_api.py [--output OUTPUT] [--verbose]

Options:
    --output, -o    Output file path (default: public_api_report.md)
    --verbose, -v   Enable verbose output
    --src-dir       Source directory (default: src)
"""

import os
import re
import sys
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple
from collections import defaultdict
from datetime import datetime


@dataclass
class PublicItem:
    """Represents a single public item in the codebase."""
    file: str
    line: int
    kind: str  # mod, struct, enum, trait, fn, use, type, const, static, impl
    name: str
    full_line: str
    visibility: str = "pub"
    module_path: str = ""
    
    def __str__(self):
        return f"{self.module_path}/{self.file}:{self.line} - {self.kind} {self.name}"


@dataclass
class ModuleSummary:
    """Summary of public items in a module."""
    module_name: str
    items: List[PublicItem] = field(default_factory=list)
    submodules: Dict[str, 'ModuleSummary'] = field(default_factory=dict)
    
    @property
    def total_count(self) -> int:
        return len(self.items) + sum(m.total_count for m in self.submodules.values())
    
    @property
    def direct_count(self) -> int:
        return len(self.items)


class PublicAPIAnalyzer:
    """Analyzes Rust source code to identify public API surface."""
    
    # Regex patterns for public items
    PUB_PATTERNS = {
        'mod': re.compile(r'^pub\s+mod\s+(\w+)'),
        'struct': re.compile(r'^pub\s+struct\s+(\w+)'),
        'enum': re.compile(r'^pub\s+enum\s+(\w+)'),
        'trait': re.compile(r'^pub\s+trait\s+(\w+)'),
        'fn': re.compile(r'^pub\s+fn\s+(\w+)'),
        'use': re.compile(r'^pub\s+use\s+(.+?)(?:;|$)'),
        'type': re.compile(r'^pub\s+type\s+(\w+)'),
        'const': re.compile(r'^pub\s+const\s+(\w+)'),
        'static': re.compile(r'^pub\s+static\s+(\w+)'),
        'impl': re.compile(r'^pub\s+impl'),
    }
    
    # Patterns to exclude (restricted visibility)
    EXCLUDE_PATTERNS = [
        re.compile(r'^pub\s*\('),  # pub(crate), pub(super), pub(in ...)
        re.compile(r'^#\[cfg\(test\)\]'),  # Test-only items
        re.compile(r'^#\[cfg\(feature\s*=\s*"bench-internals"\)\]'),  # Feature-gated
    ]
    
    def __init__(self, src_dir: str, verbose: bool = False):
        self.src_dir = Path(src_dir)
        self.verbose = verbose
        self.items: List[PublicItem] = []
        self.modules: Dict[str, ModuleSummary] = {}
        
    def find_rust_files(self) -> List[Path]:
        """Find all Rust source files in the source directory."""
        rust_files = []
        for root, dirs, files in os.walk(self.src_dir):
            # Skip target directory
            if 'target' in root:
                continue
            for file in files:
                if file.endswith('.rs'):
                    rust_files.append(Path(root) / file)
        return sorted(rust_files)
    
    def get_module_path(self, file_path: Path) -> str:
        """Extract module path from file path."""
        try:
            rel_path = file_path.relative_to(self.src_dir)
            parts = list(rel_path.parts)
            if parts[-1] == 'mod.rs':
                parts = parts[:-1]
            else:
                parts[-1] = parts[-1][:-3]  # Remove .rs
            return '/'.join(parts) if parts else 'root'
        except ValueError:
            return str(file_path)
    
    def parse_file(self, file_path: Path) -> List[PublicItem]:
        """Parse a single Rust file for public items."""
        items = []
        module_path = self.get_module_path(file_path)
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
        except Exception as e:
            if self.verbose:
                print(f"Error reading {file_path}: {e}")
            return items
        
        for line_num, line in enumerate(lines, 1):
            stripped = line.strip()
            
            # Skip comments and empty lines
            if stripped.startswith('//') or not stripped:
                continue
            
            # Check for excluded patterns
            if any(pattern.match(stripped) for pattern in self.EXCLUDE_PATTERNS):
                continue
            
            # Check for public items
            for kind, pattern in self.PUB_PATTERNS.items():
                match = pattern.match(stripped)
                if match:
                    name = match.group(1) if match.lastindex else ''
                    # Clean up use statements
                    if kind == 'use':
                        name = name.split(',')[-1].strip().split()[-1].split('::')[-1].rstrip('*')
                    
                    item = PublicItem(
                        file=str(file_path.relative_to(self.src_dir)),
                        line=line_num,
                        kind=kind,
                        name=name,
                        full_line=stripped,
                        module_path=module_path
                    )
                    items.append(item)
                    break
        
        return items
    
    def analyze(self):
        """Run the analysis on all source files."""
        rust_files = self.find_rust_files()
        
        if self.verbose:
            print(f"Found {len(rust_files)} Rust files")
        
        for file_path in rust_files:
            items = self.parse_file(file_path)
            self.items.extend(items)
            
            if self.verbose and items:
                print(f"  {file_path.relative_to(self.src_dir)}: {len(items)} public items")
        
        # Organize by top-level module
        self._organize_by_module()
    
    def _organize_by_module(self):
        """Organize items by top-level module."""
        for item in self.items:
            top_module = item.module_path.split('/')[0]
            if top_module not in self.modules:
                self.modules[top_module] = ModuleSummary(top_module)
            
            # Add to appropriate module or submodule
            if item.module_path == top_module:
                self.modules[top_module].items.append(item)
            else:
                # Find or create submodule
                parts = item.module_path.split('/')[1:]
                current = self.modules[top_module]
                for part in parts[:-1]:
                    if part not in current.submodules:
                        current.submodules[part] = ModuleSummary(part)
                    current = current.submodules[part]
                
                if parts[-1] not in current.submodules:
                    current.submodules[parts[-1]] = ModuleSummary(parts[-1])
                current.submodules[parts[-1]].items.append(item)
    
    def generate_report(self, output_file: Optional[str] = None) -> str:
        """Generate a comprehensive markdown report."""
        lines = []
        
        # Header
        lines.append("# Splice Public API Report")
        lines.append("")
        lines.append(f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        lines.append(f"**Source Directory**: {self.src_dir}")
        lines.append(f"**Total Public Items**: {len(self.items)}")
        lines.append("")
        
        # Executive Summary
        lines.append("## Executive Summary")
        lines.append("")
        lines.append("### Root Module Exports")
        
        # Find root-level items
        root_items = [i for i in self.items if i.module_path == 'root' or i.file == 'lib.rs']
        pub_mods = [i for i in root_items if i.kind == 'mod']
        pub_fns = [i for i in root_items if i.kind == 'fn']
        
        lines.append(f"- **Public modules**: {len(pub_mods)}")
        for mod in pub_mods:
            lines.append(f"  - `{mod.name}`")
        
        lines.append(f"- **Public functions**: {len(pub_fns)}")
        for fn in pub_fns:
            lines.append(f"  - `{fn.name}()`")
        
        lines.append("")
        
        # Check for pub(crate) modules
        lib_rs_path = self.src_dir / 'lib.rs'
        if lib_rs_path.exists():
            with open(lib_rs_path, 'r') as f:
                content = f.read()
                priv_mods = re.findall(r'pub\(crate\)\s+mod\s+(\w+)', content)
                if priv_mods:
                    lines.append("### Internal Modules (pub(crate))")
                    for mod in sorted(priv_mods):
                        lines.append(f"- ✅ `{mod}` - Internal only")
                    lines.append("")
        
        # Detailed breakdown
        lines.append("## Detailed Breakdown by Module")
        lines.append("")
        
        # Summary table
        lines.append("### Module Summary")
        lines.append("")
        lines.append("| Module | Items | Direct | Submodules |")
        lines.append("|--------|-------|--------|------------|")
        
        for module_name in sorted(self.modules.keys()):
            module = self.modules[module_name]
            direct = module.direct_count
            total = module.total_count
            submods = len(module.submodules)
            lines.append(f"| `{module_name}/` | {total} | {direct} | {submods} |")
        
        lines.append("")
        
        # Detailed sections
        for module_name in sorted(self.modules.keys()):
            module = self.modules[module_name]
            lines.append(f"### `{module_name}/` ({module.total_count} items)")
            lines.append("")
            
            if module.items:
                lines.append(f"**Direct items** ({len(module.items)}):")
                lines.append("")
                for item in sorted(module.items, key=lambda x: (x.kind, x.line)):
                    lines.append(f"- `{item.kind}` {item.name} (line {item.line})")
                lines.append("")
            
            if module.submodules:
                lines.append(f"**Submodules** ({len(module.submodules)}):")
                lines.append("")
                for submod_name in sorted(module.submodules.keys()):
                    submod = module.submodules[submod_name]
                    lines.append(f"- `{submod_name}/` ({submod.total_count} items)")
                    if submod.items and self.verbose:
                        for item in sorted(submod.items, key=lambda x: x.line)[:10]:
                            lines.append(f"  - `{item.kind}` {item.name}")
                        if len(submod.items) > 10:
                            lines.append(f"  - ... and {len(submod.items) - 10} more")
                lines.append("")
        
        # Visibility analysis
        lines.append("## Visibility Analysis")
        lines.append("")
        
        # Count by kind
        kind_counts = defaultdict(int)
        for item in self.items:
            kind_counts[item.kind] += 1
        
        lines.append("### By Item Kind")
        lines.append("")
        lines.append("| Kind | Count | Percentage |")
        lines.append("|------|-------|------------|")
        for kind in sorted(kind_counts.keys()):
            count = kind_counts[kind]
            pct = (count / len(self.items)) * 100
            lines.append(f"| {kind} | {count} | {pct:.1f}% |")
        lines.append("")
        
        # Recommendations
        lines.append("## Recommendations")
        lines.append("")
        lines.append("### 1. Keep Public (External API or Benchmark Usage)")
        lines.append("- `spot` - Core types")
        lines.append("- `models` - Element definitions")
        lines.append("- `solver` - Solver backends")
        lines.append("- `frontends::serde` - Network API, serialization")
        lines.append("- `sim::integration` - Integration methods")
        lines.append("")
        
        lines.append("### 2. Consider Making `pub(crate)` (Internal Only)")
        lines.append("- `frontends::spice` - SPICE parser")
        lines.append("- `frontends::kicad` - KiCad parser")
        lines.append("- `sim::autotune` - Internal tuning")
        lines.append("- `sim::newton_armijo` - Internal Newton solver")
        lines.append("")
        
        lines.append("### 3. Already Hidden")
        lines.append("- ✅ `bench_prelude` - Marked with `#[doc(hidden)]`")
        lines.append("")
        
        # Full listing
        if self.verbose:
            lines.append("## Complete Item Listing")
            lines.append("")
            for module_name in sorted(self.modules.keys()):
                module = self.modules[module_name]
                lines.append(f"### {module_name}/")
                lines.append("")
                lines.append("```")
                for item in sorted(module.items, key=lambda x: (x.module_path, x.line)):
                    lines.append(f"{item.module_path}/{item.file}:{item.line}: pub {item.kind} {item.name}")
                lines.append("```")
                lines.append("")
        
        # Footer
        lines.append("---")
        lines.append("")
        lines.append("*Generated by `scripts/analyze_public_api.py`*")
        
        report = '\n'.join(lines)
        
        if output_file:
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(report)
            if self.verbose:
                print(f"Report written to {output_file}")
        
        return report


def main():
    parser = argparse.ArgumentParser(
        description='Analyze public API surface of Splice Rust codebase'
    )
    parser.add_argument(
        '-o', '--output',
        default='notes/public_api_report.md',
        help='Output file path (default: notes/public_api_report.md)'
    )
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Enable verbose output'
    )
    parser.add_argument(
        '-s', '--src-dir',
        default='src',
        help='Source directory (default: src)'
    )
    
    args = parser.parse_args()
    
    # Check if source directory exists
    if not os.path.isdir(args.src_dir):
        print(f"Error: Source directory '{args.src_dir}' not found")
        sys.exit(1)
    
    # Run analysis
    analyzer = PublicAPIAnalyzer(args.src_dir, verbose=args.verbose)
    analyzer.analyze()
    
    # Generate report
    report = analyzer.generate_report(args.output)
    
    # Print summary
    print(f"\n[OK] Analysis complete!")
    print(f"   Total public items: {len(analyzer.items)}")
    print(f"   Modules analyzed: {len(analyzer.modules)}")
    print(f"   Report: {args.output}")
    print(f"\nTop-level modules:")
    for module_name in sorted(analyzer.modules.keys()):
        module = analyzer.modules[module_name]
        print(f"   - {module_name}/: {module.total_count} items")


if __name__ == '__main__':
    main()
