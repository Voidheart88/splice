import os
from tree_sitter import Language, Parser, Query, QueryCursor
import tree_sitter_rust as tsrust

# Lade die Rust-Sprachdefinition
RUST_LANG = Language(tsrust.language())
parser = Parser(RUST_LANG)

query = Query(
    RUST_LANG,
    """
    (function_item name: (identifier) @func.name)
    (call_expression function: (identifier) @call.func)
    """
)

def find_enclosing_function(node):
    while node.parent:
        node = node.parent
        if node.type == "function_item":
            return node.text.decode("utf-8")
    return None

def extract_functions_and_calls(tree):
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
    callgraph = []

    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r') as f:
                        code = f.read()
                    tree = parser.parse(bytes(code, 'utf-8'))
                    functions, calls = extract_functions_and_calls(tree)
                    callgraph.extend(calls)
                    print(f"Verarbeite {file_path}...")
                except FileNotFoundError:
                    print(f"Datei nicht gefunden: {file_path}")
                except Exception as e:
                    print(f"Fehler beim Parsen von {file_path}: {str(e)}")

    return callgraph

def write_dot_file(callgraph, output_file):
    with open(output_file, 'w') as f:
        f.write("digraph callgraph {\n")
        for caller, callee in callgraph:
            f.write(f'    "{caller}" -> "{callee}";\n')
        f.write("}\n")

if __name__ == "__main__":
    callgraph = generate_callgraph("src")
    write_dot_file(callgraph, "callgraph.dot")
    print("Callgraph generiert: callgraph.dot")
