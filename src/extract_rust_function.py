import sys
import os
import re

UNWANTED_FUNCTIONS = {"main", "panic", "eh_personality"}


def clean_source(src):
    # Remove crate-level and other attributes like #[no_mangle], #[panic_handler]
    src = re.sub(r"#!\[.*?\]", "", src)
    src = re.sub(r"#\[.*?\]", "", src)

    # Remove use statements
    src = re.sub(r"^\s*use\s+.*?;\s*", "", src, flags=re.MULTILINE)

    # Remove extern "C"
    src = re.sub(r"extern\s+\"C\"\s*", "", src)

    return src


def extract_functions(src):
    functions = []
    i = 0
    length = len(src)

    while i < length:
        fn_pos = src.find("fn ", i)
        if fn_pos == -1:
            break

        # Match function name
        name_match = re.match(r"fn\s+([A-Za-z0-9_]+)", src[fn_pos:])
        if not name_match:
            i = fn_pos + 3
            continue

        fn_name = name_match.group(1)

        # Skip unwanted functions
        if fn_name in UNWANTED_FUNCTIONS:
            i = fn_pos + 4
            continue

        # Find start of function body
        brace_pos = src.find("{", fn_pos)
        if brace_pos == -1:
            break

        # Brace matching
        depth = 0
        end = brace_pos
        while end < length:
            if src[end] == "{":
                depth += 1
            elif src[end] == "}":
                depth -= 1
                if depth == 0:
                    end += 1
                    break
            end += 1

        fn_text = src[fn_pos:end].strip()
        functions.append(fn_text)

        i = end

    return "\n\n\n".join(functions)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python extract_rust_fn.py <source.rs>")
        sys.exit(1)

    input_file = sys.argv[1]
    with open(input_file, "r") as f:
        src = f.read()

    src = clean_source(src)
    cleaned = extract_functions(src)

    base, ext = os.path.splitext(input_file)
    output_file = f"{base}_clean.rs"

    with open(output_file, "w") as f:
        f.write(cleaned + "\n")

    print(f"Cleaned Rust file written to: {output_file}")
