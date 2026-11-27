#!/usr/bin/env python3
"""
Data Flow Analyzer for Splice Circuit Simulator

This tool analyzes the Rust codebase to track how data is transformed
through the simulation pipeline. It identifies:

1. Data structures (structs, enums) and their fields
2. Conversion functions (From, Into, custom converters)
3. Data flow between modules
4. Field-level mappings between structures

Output: Interactive HTML visualization + Mermaid diagrams
"""

import os
import re
import json
from dataclasses import dataclass, field
from typing import Dict, List, Set, Tuple, Optional
from pathlib import Path
from collections import defaultdict


@dataclass
class DataStructure:
    """Represents a Rust struct or enum"""
    name: str
    kind: str  # 'struct' or 'enum'
    module: str
    fields: Dict[str, str] = field(default_factory=dict)  # field_name -> type
    generics: List[str] = field(default_factory=list)
    line_number: int = 0
    file_path: str = ""
    
    def full_name(self) -> str:
        return f"{self.module}::{self.name}"


@dataclass
class Transformation:
    """Represents a data transformation (function that converts data)"""
    name: str
    from_type: str
    to_type: str
    module: str
    file_path: str
    line_number: int = 0
    transformation_type: str = ""  # 'From', 'Into', 'method', 'function'
    body_snippet: str = ""


@dataclass
class DataFlow:
    """Represents a complete data flow path"""
    source: str
    target: str
    path: List[str]  # List of transformation names
    modules: List[str]  # Modules involved


class DataFlowAnalyzer:
    """Main analyzer class"""
    
    def __init__(self, src_dir: str = "src"):
        self.src_dir = Path(src_dir)
        self.structures: Dict[str, DataStructure] = {}
        self.transformations: List[Transformation] = []
        self.module_deps: Dict[str, Set[str]] = defaultdict(set)
        
    def analyze(self):
        """Run the complete analysis"""
        print("[1/4] Analyzing Splice data flow...")
        
        print("  [2/4] Parsing Rust source files...")
        self._analyze_files()
        
        print("  [3/4] Analyzing transformations...")
        self._analyze_transformations()
        
        print("  [4/4] Building data flows...")
        self._build_data_flows()
        
        print(f"\n=== Analysis Complete ===")
        print(f"  - Data structures found: {len(self.structures)}")
        print(f"  - Transformations found: {len(self.transformations)}")
        
        # Print main pipeline summary
        self._print_pipeline_summary()
        
    def _get_module_name(self, file_path: Path) -> str:
        """Extract module name from file path"""
        try:
            rel_path = file_path.relative_to(self.src_dir)
            parts = list(rel_path.parts)
            if parts[-1] == 'mod.rs':
                parts = parts[:-1]
            else:
                parts[-1] = parts[-1][:-3]  # Remove .rs
            
            return '::'.join(parts) if parts else 'crate'
        except ValueError:
            return 'unknown'
    
    def _analyze_files(self):
        """Parse all Rust files to extract structures"""
        file_count = 0
        for rs_file in self.src_dir.rglob("*.rs"):
            try:
                file_count += 1
                with open(rs_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                module_name = self._get_module_name(rs_file)
                self._extract_structs(content, module_name, str(rs_file))
                
            except Exception as e:
                print(f"  Warning: Error parsing {rs_file}: {e}")
        
        print(f"    Processed {file_count} Rust files")
    
    def _extract_structs(self, content: str, module_name: str, file_path: str):
        """Extract struct and enum definitions from Rust code"""
        lines = content.split('\n')
        
        i = 0
        while i < len(lines):
            line = lines[i]
            stripped = line.strip()
            
            # Skip comments
            if stripped.startswith('//') or stripped.startswith('//!'):
                i += 1
                continue
            
            # Match struct definitions - handle attributes like #[derive(...)]
            struct_match = re.search(r'(?:pub(?:\([^)]*\))?\s+)?struct\s+(\w+)(?:<([^>]+)>)?\s*\{', stripped)
            if struct_match:
                name = struct_match.group(1)
                generics = struct_match.group(2)
                
                struct = DataStructure(
                    name=name,
                    kind='struct',
                    module=module_name,
                    generics=[g.strip() for g in generics.split(',')] if generics else [],
                    file_path=file_path,
                    line_number=i + 1
                )
                name = struct_match.group(1)
                generics = struct_match.group(2)
                
                struct = DataStructure(
                    name=name,
                    kind='struct',
                    module=module_name,
                    generics=[g.strip() for g in generics.split(',')] if generics else [],
                    file_path=file_path,
                    line_number=i + 1
                )
                
                # Extract fields - look for the struct body
                brace_count = stripped.count('{') - stripped.count('}')
                field_lines = [stripped]
                j = i + 1
                
                while brace_count > 0 and j < len(lines):
                    field_lines.append(lines[j])
                    brace_count += lines[j].count('{') - lines[j].count('}')
                    j += 1
                
                # Parse fields from collected lines
                for field_line in field_lines:
                    field_match = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(\w+):\s*([^,\s}]+)', field_line)
                    if field_match:
                        field_name = field_match.group(1)
                        field_type = field_match.group(2).rstrip(',')
                        struct.fields[field_name] = field_type
                
                self.structures[struct.full_name()] = struct
                i = j
                continue
            
            # Match enum definitions - handle attributes
            enum_match = re.search(r'(?:pub(?:\([^)]*\))?\s+)?enum\s+(\w+)(?:<([^>]+)>)?\s*\{', stripped)
            if enum_match:
                name = enum_match.group(1)
                generics = enum_match.group(2)
                
                enum_struct = DataStructure(
                    name=name,
                    kind='enum',
                    module=module_name,
                    generics=[g.strip() for g in generics.split(',')] if generics else [],
                    file_path=file_path,
                    line_number=i + 1
                )
                
                self.structures[enum_struct.full_name()] = enum_struct
                i += 1
                continue
            
            i += 1
    
    def _analyze_transformations(self):
        """Find all data transformations in the codebase"""
        # Track both simple one-liners and multi-line impl blocks
        for rs_file in self.src_dir.rglob("*.rs"):
            try:
                with open(rs_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                    lines = content.split('\n')
                
                module_name = self._get_module_name(rs_file)
                
                # Look for From/Into implementations (can span multiple lines)
                self._find_from_impls(content, lines, module_name, str(rs_file))
                
                # Look for trait implementation methods that convert data
                self._find_conversion_methods(content, lines, module_name, str(rs_file))
                
                # Look for conversion functions
                for i, line in enumerate(lines):
                    stripped = line.strip()
                    
                    # Skip comments
                    if stripped.startswith('//'):
                        continue
                    
                    # Check for conversion functions: fn xxx_to_yyy
                    conv_match = re.search(r'fn\s+(\w+_to_\w+|convert_\w+|as_\w+)\s*\([^)]*\)\s*(?:->\s*([^\{]+))?', stripped)
                    if conv_match and '->' in stripped:
                        func_name = conv_match.group(1)
                        return_type = conv_match.group(2).strip().rstrip('{').strip() if conv_match.group(2) else "Unknown"
                        
                        # Infer from_type from function name
                        from_type = "Unknown"
                        if '_to_' in func_name:
                            from_type = func_name.split('_to_')[0]
                        elif func_name.startswith('convert_'):
                            from_type = func_name[8:]
                        
                        self.transformations.append(Transformation(
                            name=func_name,
                            from_type=from_type,
                            to_type=return_type,
                            module=module_name,
                            file_path=str(rs_file),
                            line_number=i + 1,
                            transformation_type='function',
                            body_snippet=stripped
                        ))
                        
            except Exception as e:
                print(f"  Warning: Error analyzing {rs_file}: {e}")
    
    def _find_conversion_methods(self, content: str, lines: List[str], module_name: str, file_path: str):
        """Find methods in trait implementations that convert data (e.g., fn simulation() -> Result<Simulation, ...>)"""
        # Look for impl blocks for Frontend trait
        frontend_impl = re.compile(r'impl\s+Frontend\s+for\s+(\w+)')
        
        i = 0
        while i < len(lines):
            line = lines[i]
            match = frontend_impl.search(line)
            if match:
                impl_type = match.group(1)
                # Look for simulation() method in this impl block
                impl_body = self._extract_impl_body(lines, i)
                
                # Look for fn simulation(&self) -> Result<Simulation, ...>
                sim_method = re.search(r'fn\s+simulation\s*\(&self\)\s*->\s*Result<(\w+)', impl_body)
                if sim_method:
                    return_type = sim_method.group(1)
                    self.transformations.append(Transformation(
                        name="simulation()",
                        from_type=impl_type,
                        to_type=return_type,
                        module=module_name,
                        file_path=file_path,
                        line_number=i + 1,
                        transformation_type='method',
                        body_snippet="fn simulation(&self) -> Result<" + return_type + ", ...>"
                    ))
                    print(f"    [Conversion Method] {impl_type}.simulation() -> {return_type}")
            i += 1
    
    def _find_from_impls(self, content: str, lines: List[str], module_name: str, file_path: str):
        """Find From/Into implementations, including multi-line ones"""
        # Pattern for impl blocks - handle complex type expressions
        impl_pattern = re.compile(r'impl\s+(?:<[^>]+>\s+)?(From|Into)<([^>]+)>\s+for\s+(\w+(?:<[^>]+>)?)')
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Check for impl From/Into
            match = impl_pattern.search(line)
            if match:
                trait = match.group(1)
                if trait == 'From':
                    from_type = match.group(2).strip()
                    to_type = match.group(3).strip()
                else:  # Into
                    to_type = match.group(2).strip()
                    from_type = match.group(3).strip()
                
                # Extract the full implementation body
                impl_body = self._extract_impl_body(lines, i)
                
                # Always record the transformation
                self.transformations.append(Transformation(
                    name=f"{trait}<{from_type}>",
                    from_type=from_type,
                    to_type=to_type,
                    module=module_name,
                    file_path=file_path,
                    line_number=i + 1,
                    transformation_type=trait,
                    body_snippet=impl_body[:200] if impl_body else line.strip()
                ))
                
                # For non-error transformations, extract detailed field mappings
                if not self._is_error_type(from_type) and not self._is_error_type(to_type):
                    self._extract_field_mappings(impl_body, from_type, to_type, module_name, file_path, i + 1)
            i += 1
    
    def _extract_impl_body(self, lines: List[str], start_idx: int) -> str:
        """Extract the full body of an impl block"""
        brace_count = 0
        body_lines = []
        found_open = False
        
        for i in range(start_idx, min(start_idx + 100, len(lines))):
            line = lines[i]
            if '{' in line:
                found_open = True
                brace_count += line.count('{') - line.count('}')
            elif found_open:
                brace_count += line.count('{') - line.count('}')
            
            if found_open:
                body_lines.append(line)
            
            if found_open and brace_count <= 0:
                break
        
        return '\n'.join(body_lines)
    
    def _extract_field_mappings(self, impl_body: str, from_type: str, to_type: str,
                                 module_name: str, file_path: str, line_number: int):
        """Extract field-level mappings from a conversion implementation"""
        if not impl_body:
            return
        
        print(f"    [Field Mapping] {from_type} -> {to_type}:")
        
        # Look for struct construction: TargetStruct { field: value, ... }
        construct_pattern = re.compile(rf'{to_type}\s*\{{\s*([^}}]+)\}}', re.DOTALL)
        match = construct_pattern.search(impl_body)
        
        if match:
            fields_str = match.group(1)
            # Extract field assignments
            field_assigns = re.findall(r'(\w+):\s*([^,\n}]+)', fields_str)
            for field_name, field_value in field_assigns[:5]:  # Show first 5
                field_value_clean = field_value.strip().replace('\n', ' ').strip()
                print(f"      .{field_name} = {field_value_clean}")
        else:
            # Look for match expressions (enum conversions)
            match_expr = re.search(r'match\s+(\w+)\s*\{([^}]+(?:\{[^}]*\}[^}]*)*)\}', impl_body, re.DOTALL)
            if match_expr:
                match_var = match_expr.group(1)
                arms_str = match_expr.group(2)
                # Extract match arms
                arms = re.findall(r'(\w+(?:::\w+)*)\s*(?:\([^)]*\))?\s*=>\s*([^,\n]+)', arms_str)
                for arm_pattern, result in arms[:5]:
                    result_clean = result.strip().replace('\n', ' ').strip()[:60]
                    print(f"      {arm_pattern}(...) -> {result_clean}")
    
    def _is_error_type(self, type_name: str) -> bool:
        """Check if a type is error-related"""
        error_indicators = ['Error', 'Err', 'ParseError', 'IoError', 'SimError']
        return any(indicator in type_name for indicator in error_indicators)
    
    def _extract_conversion_body(self, lines: List[str], start_idx: int, from_type: str, to_type: str, 
                                  module_name: str, file_path: str):
        """Extract the conversion implementation to analyze field mappings"""
        brace_count = 0
        body_lines = []
        found_open = False
        
        for i in range(start_idx, min(start_idx + 50, len(lines))):
            line = lines[i]
            if '{' in line:
                found_open = True
                brace_count += line.count('{') - line.count('}')
            elif found_open:
                brace_count += line.count('{') - line.count('}')
            
            if found_open:
                body_lines.append(line)
            
            if found_open and brace_count <= 0:
                break
        
        # Look for field mappings in the body
        body = '\n'.join(body_lines)
        
        # Look for struct construction patterns: StructName { field1: value1, ... }
        struct_construct = re.search(rf'{to_type}\s*\{{\s*([^}}]+)\}}', body, re.DOTALL)
        if struct_construct:
            fields_str = struct_construct.group(1)
            # Extract field assignments
            field_assigns = re.findall(r'(\w+):\s*([^,\n]+)', fields_str)
            for field_name, field_value in field_assigns:
                # This shows how from_type fields map to to_type fields
                pass  # Could store this for detailed field mapping report
    
    def _build_data_flows(self):
        """Build complete data flow paths"""
        # Build a graph of transformations
        graph = defaultdict(list)
        for trans in self.transformations:
            graph[trans.from_type].append((trans.to_type, trans.name))
        
        # Also add structure relationships (field types)
        for struct_name, struct in self.structures.items():
            for field_name, field_type in struct.fields.items():
                # Extract base type from complex types like Option<T>, Vec<T>, etc.
                base_type = self._extract_base_type(field_type)
                if base_type and base_type != struct_name:
                    graph[base_type].append((struct_name, f'field:{field_name}'))
        
        # Find paths from common source types to result types
        source_types = ['SerdeCircuit', 'String', 'File']
        target_types = ['SimulationResults', 'Simulation']
        
        # Simple BFS to find paths
        for source in source_types:
            for target in target_types:
                path = self._find_path_bfs(graph, source, target)
                if path:
                    print(f"\n[+] Data flow: {source} -> {target}")
                    for step in path:
                        print(f"    {step}")
    
    def _extract_base_type(self, type_str: str) -> Optional[str]:
        """Extract base type from complex type expressions"""
        # Remove references and mutability
        type_str = type_str.strip()
        if type_str.startswith('&'):
            type_str = type_str[1:].lstrip('mut ').strip()
        
        # Handle Option<T>, Vec<T>, etc.
        match = re.match(r'(\w+)::?<[^>]+>', type_str)
        if match:
            return match.group(1)
        
        # Handle simple types
        match = re.match(r'(\w+)', type_str)
        if match:
            return match.group(1)
        
        return None
    
    def _find_path_bfs(self, graph, start, end, max_depth=6):
        """Find shortest path using BFS"""
        from collections import deque
        
        queue = deque([(start, [start])])
        visited = {start}
        
        while queue:
            current, path = queue.popleft()
            
            if len(path) > max_depth:
                continue
            
            if current == end:
                return path
            
            for next_type, transform_name in graph.get(current, []):
                if next_type not in visited:
                    visited.add(next_type)
                    queue.append((next_type, path + [f"{next_type} ({transform_name})"]))
        
        return None
    
    def _print_pipeline_summary(self):
        """Print a summary of the main simulation pipeline"""
        print("\n=== Main Simulation Pipeline ===")
        
        # Key structures in the pipeline
        pipeline_stages = [
            ('Input', ['SerdeCircuit', 'SerdeElement', 'SerdeFrontend']),
            ('Core', ['Simulation', 'Element', 'Variable']),
            ('Solver', ['Simulator', 'Solver', 'IntegrationMethod']),
            ('Output', ['SimulationResults', 'CsvBackend', 'PlotBackend']),
        ]
        
        for stage_name, structures in pipeline_stages:
            found = [s for s in structures if any(s in struct for struct in self.structures.keys())]
            if found:
                print(f"\n{stage_name}:")
                for struct_name in found:
                    # Find the full struct name
                    for full_name, struct in self.structures.items():
                        if struct.name == struct_name:
                            field_count = len(struct.fields)
                            print(f"  - {struct_name} ({field_count} fields) [{struct.module}]")
                            break
        
        # Print actual data transformations (non-error)
        print("\n=== Data Transformations (From Traits) ===")
        data_transforms = [t for t in self.transformations 
                          if t.transformation_type == 'From' 
                          and not self._is_error_type(t.from_type)
                          and not self._is_error_type(t.to_type)]
        
        if data_transforms:
            for trans in data_transforms[:20]:  # Show first 20
                print(f"  {trans.from_type} -> {trans.to_type}")
                print(f"    Location: {trans.module}")
        else:
            print("  No non-error From transformations found")
            print("  (May need to expand search patterns)")
    
    def generate_mermaid(self) -> str:
        """Generate Mermaid.js diagram focusing on main data flow"""
        lines = ["flowchart TD"]
        lines.append("    classDef struct fill:#e1f5ff,stroke:#01579b,stroke-width:2px")
        lines.append("    classDef enum fill:#fff3e0,stroke:#e65100,stroke-width:2px")
        lines.append("    classDef transform fill:#f3e5f5,stroke:#4a148c,stroke-width:2px,stroke-dasharray: 5 5")
        lines.append("    classDef pipeline fill:#e8f5e9,stroke:#1b5e20,stroke-width:3px")
        lines.append("")
        
        # Define the main simulation pipeline structures
        pipeline_structs = {
            'SerdeCircuit', 'SerdeElement', 'Simulation', 'Element', 
            'Simulator', 'SimulationResults', 'Variable', 'Numeric'
        }
        
        # Add pipeline structures with special styling
        for struct_name, struct in self.structures.items():
            if struct.name in pipeline_structs:
                lines.append(f"    {struct.name}[\"{struct.name}\"]:::pipeline")
        
        # Add other non-error structures
        for struct_name, struct in self.structures.items():
            if struct.name not in pipeline_structs and not self._is_error_type(struct.name):
                lines.append(f"    {struct.name}[\"{struct.name}\"]:::struct")
        
        lines.append("")
        
        # Add only non-error transformations
        for trans in self.transformations:
            from_name = trans.from_type.split('<')[0].split('&')[-1].strip()
            to_name = trans.to_type.split('<')[0].split('&')[-1].strip()
            
            # Skip error-related transformations
            if self._is_error_type(from_name) or self._is_error_type(to_name):
                continue
            
            # Show transformations between known structures
            if from_name in [s.name for s in self.structures.values()] or \
               to_name in [s.name for s in self.structures.values()]:
                lines.append(f"    {from_name} -->|{trans.transformation_type}| {to_name}")
        
        return '\n'.join(lines)
    
    def generate_html_report(self, output_path: str = "notes/data_flow_report.html"):
        """Generate interactive HTML report"""
        mermaid_diagram = self.generate_mermaid()
        
        # Prepare data for interactive visualization
        structures_data = []
        for struct in self.structures.values():
            structures_data.append({
                'name': struct.name,
                'kind': struct.kind,
                'module': struct.module,
                'fields': struct.fields,
                'generics': struct.generics
            })
        
        transformations_data = []
        for trans in self.transformations:
            transformations_data.append({
                'name': trans.name,
                'from_type': trans.from_type,
                'to_type': trans.to_type,
                'module': trans.module,
                'type': trans.transformation_type,
                'snippet': trans.body_snippet
            })
        
        html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Splice Data Flow Analysis</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #f5f5f5; }}
        .container {{ max-width: 1400px; margin: 0 auto; padding: 20px; }}
        h1 {{ color: #333; margin-bottom: 20px; border-bottom: 3px solid #007acc; padding-bottom: 10px; }}
        h2 {{ color: #555; margin: 30px 0 15px; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }}
        .stat-card {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .stat-number {{ font-size: 2.5em; color: #007acc; font-weight: bold; }}
        .stat-label {{ color: #666; margin-top: 5px; }}
        .diagram-container {{ background: white; padding: 20px; border-radius: 8px; margin: 20px 0; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        table {{ width: 100%; border-collapse: collapse; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        th {{ background: #007acc; color: white; padding: 12px; text-align: left; }}
        td {{ padding: 12px; border-bottom: 1px solid #eee; }}
        tr:hover {{ background: #f5f5f5; }}
        .struct-tag {{ display: inline-block; padding: 3px 8px; border-radius: 4px; font-size: 0.85em; }}
        .struct-tag.struct {{ background: #e1f5ff; color: #01579b; }}
        .struct-tag.enum {{ background: #fff3e0; color: #e65100; }}
        .transform-tag {{ background: #f3e5f5; color: #4a148c; }}
        .module-tag {{ background: #e8f5e9; color: #1b5e20; font-size: 0.85em; padding: 2px 6px; border-radius: 3px; }}
        .fields-list {{ font-size: 0.9em; color: #666; }}
        .field {{ margin: 2px 0; }}
        .field-name {{ color: #007acc; }}
        .field-type {{ color: #d32f2f; }}
        .snippet {{ background: #f5f5f5; padding: 8px; border-radius: 4px; font-family: 'Consolas', monospace; font-size: 0.85em; overflow-x: auto; }}
        .filter-section {{ background: white; padding: 20px; border-radius: 8px; margin: 20px 0; }}
        .filter-section input, .filter-section select {{ padding: 8px; margin: 5px; border-radius: 4px; border: 1px solid #ddd; }}
        .hidden {{ display: none; }}
        .pipeline {{ background: linear-gradient(90deg, #e3f2fd 0%, #bbdefb 100%); padding: 20px; border-radius: 8px; margin: 20px 0; }}
        .pipeline-step {{ display: inline-block; padding: 10px 20px; margin: 5px; background: white; border-radius: 4px; border: 2px solid #007acc; }}
        .pipeline-arrow {{ font-size: 1.5em; color: #007acc; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Splice Data Flow Analysis</h1>
        
        <div class="stats">
            <div class="stat-card">
                <div class="stat-number">{len(self.structures)}</div>
                <div class="stat-label">Data Structures</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{len(self.transformations)}</div>
                <div class="stat-label">Transformations</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{len(set(s.module for s in self.structures.values()))}</div>
                <div class="stat-label">Modules Analyzed</div>
            </div>
        </div>
        
        <h2>Main Simulation Pipeline</h2>
        <div class="pipeline">
            <div class="pipeline-step">File Input<br><small>(SPICE/YAML/JSON)</small></div>
            <span class="pipeline-arrow">&#8594;</span>
            <div class="pipeline-step">Frontend<br><small>(Parser)</small></div>
            <span class="pipeline-arrow">&#8594;</span>
            <div class="pipeline-step">Simulation<br><small>(Elements + Commands)</small></div>
            <span class="pipeline-arrow">&#8594;</span>
            <div class="pipeline-step">Simulator<br><small>(Matrix Builder)</small></div>
            <span class="pipeline-arrow">&#8594;</span>
            <div class="pipeline-step">Solver<br><small>(Faer/Nalgebra/RSparse)</small></div>
            <span class="pipeline-arrow">&#8594;</span>
            <div class="pipeline-step">Results<br><small>(CSV/Plot/Network)</small></div>
        </div>
        
        <h2>Data Flow Diagram</h2>
        <div class="diagram-container">
            <div class="mermaid">
{mermaid_diagram}
            </div>
        </div>
        
        <h2>Data Structures ({len(structures_data)} found)</h2>
        <div class="filter-section">
            <input type="text" id="structFilter" placeholder="Filter structures..." onkeyup="filterTable('structTable', 0)">
            <select id="structKindFilter" onchange="filterByKind()">
                <option value="">All Kinds</option>
                <option value="struct">Structs</option>
                <option value="enum">Enums</option>
            </select>
        </div>
        <table id="structTable">
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Kind</th>
                    <th>Module</th>
                    <th>Fields</th>
                </tr>
            </thead>
            <tbody>
                {''.join(f'''
                <tr data-kind="{s['kind']}">
                    <td><strong>{s['name']}</strong></td>
                    <td><span class="struct-tag {s['kind']}">{s['kind'].upper()}</span></td>
                    <td><span class="module-tag">{s['module']}</span></td>
                    <td class="fields-list">
                        {''.join(f'<div class="field"><span class="field-name">{k}</span>: <span class="field-type">{v}</span></div>' for k, v in list(s['fields'].items())[:5])}
                        {'<div style="color: #999;">... and more</div>' if len(s['fields']) > 5 else ''}
                    </td>
                </tr>
                ''' for s in sorted(structures_data, key=lambda x: x['name'])[:100])}
            </tbody>
        </table>
        
        <h2>Transformations ({len(transformations_data)} found)</h2>
        <div class="filter-section">
            <input type="text" id="transformFilter" placeholder="Filter transformations..." onkeyup="filterTable('transformTable', 0)">
        </div>
        <table id="transformTable">
            <thead>
                <tr>
                    <th>Function</th>
                    <th>Type</th>
                    <th>From &rarr; To</th>
                    <th>Module</th>
                    <th>Code</th>
                </tr>
            </thead>
            <tbody>
                {''.join(f'''
                <tr>
                    <td><strong>{t['name']}</strong></td>
                    <td><span class="transform-tag">{t['type']}</span></td>
                    <td>{t['from_type']} &rarr; {t['to_type']}</td>
                    <td><span class="module-tag">{t['module']}</span></td>
                    <td><div class="snippet">{t['snippet'][:100]}{'...' if len(t['snippet']) > 100 else ''}</div></td>
                </tr>
                ''' for t in transformations_data[:100])}
            </tbody>
        </table>
    </div>
    
    <script>
        mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
        
        function filterTable(tableId, column) {{
            const filter = document.getElementById(tableId === 'structTable' ? 'structFilter' : 'transformFilter');
            const filterValue = filter.value.toUpperCase();
            const table = document.getElementById(tableId);
            const tr = table.getElementsByTagName('tr');
            
            for (let i = 1; i < tr.length; i++) {{
                const td = tr[i].getElementsByTagName('td')[column];
                if (td) {{
                    const txtValue = td.textContent || td.innerText;
                    tr[i].style.display = txtValue.toUpperCase().indexOf(filterValue) > -1 ? '' : 'none';
                }}
            }}
        }}
        
        function filterByKind() {{
            const filter = document.getElementById('structKindFilter');
            const selectedKind = filter.value;
            const rows = document.querySelectorAll('#structTable tbody tr');
            
            rows.forEach(row => {{
                if (!selectedKind || row.dataset.kind === selectedKind) {{
                    row.style.display = '';
                }} else {{
                    row.style.display = 'none';
                }}
            }});
        }}
    </script>
</body>
</html>
"""
        
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(html)
        
        print(f"\n[OK] HTML report generated: {output_path}")
        return output_path
    
    def export_json(self, output_path: str = "notes/data_flow_analysis.json"):
        """Export analysis as JSON for further processing"""
        data = {
            'structures': {
                name: {
                    'name': s.name,
                    'kind': s.kind,
                    'module': s.module,
                    'fields': s.fields,
                    'generics': s.generics,
                    'file': s.file_path,
                    'line': s.line_number
                }
                for name, s in self.structures.items()
            },
            'transformations': [
                {
                    'name': t.name,
                    'from_type': t.from_type,
                    'to_type': t.to_type,
                    'module': t.module,
                    'type': t.transformation_type,
                    'file': t.file_path,
                    'line': t.line_number,
                    'snippet': t.body_snippet
                }
                for t in self.transformations
            ]
        }
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2)
        
        print(f"[OK] JSON export: {output_path}")
        return output_path


def main():
    """Main entry point"""
    import argparse
    from pathlib import Path
    
    parser = argparse.ArgumentParser(description='Analyze data flow in Splice codebase')
    parser.add_argument('--src', default=None, help='Source directory')
    parser.add_argument('--output', default='notes/data_flow_report.html', help='Output HTML file (default: notes/data_flow_report.html)')
    parser.add_argument('--json', action='store_true', help='Also export JSON')
    parser.add_argument('--mermaid', action='store_true', help='Print Mermaid diagram')
    
    args = parser.parse_args()
    
    # Auto-detect src directory
    src_dir = args.src
    if not src_dir:
        # Try relative to script location
        script_dir = Path(__file__).parent
        if (script_dir.parent / 'src').exists():
            src_dir = str(script_dir.parent / 'src')
        elif Path('src').exists():
            src_dir = 'src'
        else:
            print("Error: Could not find src directory")
            print("Use --src to specify the source directory")
            return
    
    analyzer = DataFlowAnalyzer(src_dir)
    analyzer.analyze()
    
    # Generate reports
    analyzer.generate_html_report(args.output)
    
    if args.json:
        analyzer.export_json()
    
    if args.mermaid:
        print("\n=== Mermaid Diagram ===")
        print(analyzer.generate_mermaid())


if __name__ == '__main__':
    main()
