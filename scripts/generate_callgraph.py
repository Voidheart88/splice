#!/usr/bin/env python3
"""
Call Graph Generator for Splice Circuit Simulator

This script analyzes Rust source code to generate a call graph
showing which functions call which other functions.

Output: DOT file format (can be visualized with Graphviz)
"""

import os
import argparse
from pathlib import Path

try:
    from tree_sitter import Language, Parser, Query, QueryCursor
    import tree_sitter_rust as tsrust
    
    # Load Rust language definition
    RUST_LANG = Language(tsrust.language())
    parser = Parser(RUST_LANG)
    
    query = Query(
        RUST_LANG,
        """
        (function_item name: (identifier) @func.name)
        (call_expression function: (identifier) @call.func)
        """
    )
    
    TREE_SITTER_AVAILABLE = True
except ImportError:
    print("Warning: tree_sitter or tree_sitter_rust not installed.")
    print("Install with: pip install tree-sitter tree-sitter-rust")
    TREE_SITTER_AVAILABLE = False


def find_enclosing_function(node):
    """Find the name of the enclosing function for a node."""
    while node.parent:
        node = node.parent
        if node.type == "function_item":
            return node.text.decode("utf-8")
    return None


def extract_functions_and_calls(tree):
    """Extract function definitions and calls from a parse tree."""
    functions = set()
    calls = []

    query_cursor = QueryCursor(query)
    captures = query_cursor.captures(tree.root_node)

    for node, tag in captures.items():
        if node == "func.name":
            func_name = tag.text.decode("utf-8")
            functions.add(func_name)
        elif node == "call.func":
            caller_func = find_enclosing_function(node)
            if caller_func:
                callee_func = tag.text.decode("utf-8")
                calls.append((caller_func, callee_func))

    return functions, calls


def generate_callgraph(directory):
    """Generate call graph from all Rust files in directory."""
    callgraph = []

    for root, _, files in os.walk(directory):
        # Skip target directory
        if 'target' in root:
            continue
            
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        code = f.read()
                    tree = parser.parse(bytes(code, 'utf-8'))
                    functions, calls = extract_functions_and_calls(tree)
                    callgraph.extend(calls)
                    print(f"  Processing {file_path}...")
                except FileNotFoundError:
                    print(f"  File not found: {file_path}")
                except Exception as e:
                    print(f"  Error parsing {file_path}: {str(e)}")

    return callgraph


def write_dot_file(callgraph, output_file):
    """Write call graph to DOT file format."""
    # Ensure output directory exists
    output_path = Path(output_file)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("digraph callgraph {\n")
        f.write("  rankdir=LR;\n")
        f.write("  node [shape=box];\n")
        for caller, callee in callgraph:
            f.write(f'    "{caller}" -> "{callee}";\n')
        f.write("}\n")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Generate call graph from Splice Rust codebase'
    )
    parser.add_argument(
        '-s', '--src',
        default='src',
        help='Source directory (default: src)'
    )
    parser.add_argument(
        '-o', '--output',
        default='notes/callgraph.dot',
        help='Output DOT file (default: notes/callgraph.dot)'
    )
    
    args = parser.parse_args()
    
    if not TREE_SITTER_AVAILABLE:
        print("\nError: tree_sitter not available. Cannot generate call graph.")
        print("Install dependencies: pip install tree-sitter tree-sitter-rust")
        return 1
    
    # Check if source directory exists
    if not os.path.isdir(args.src):
        print(f"Error: Source directory '{args.src}' not found")
        return 1
    
    print(f"Analyzing Rust code in {args.src}/...")
    callgraph = generate_callgraph(args.src)
    
    print(f"\n[OK] Found {len(callgraph)} function calls")
    write_dot_file(callgraph, args.output)
    print(f"[OK] DOT file written to: {args.output}")
    print(f"\nTip: Convert to PNG with: dot -Tpng {args.output} -o callgraph.png")
    
    return 0


if __name__ == "__main__":
    exit(main())
