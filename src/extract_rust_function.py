import sys
import os
import re

UNWANTED_FUNCTIONS = {"main", "panic", "eh_personality"}


def clean_source(src: str) -> str:
    # Remove crate-level attributes like #![no_std], #![feature(...)]
    src = re.sub(r"#!\[.*?\]", "", src)

    # Remove function-level attributes like #[no_mangle], #[panic_handler], #[lang = "..."]
    src = re.sub(r"#\[.*?\]", "", src)

    # Remove use statements
    src = re.sub(r"^\s*use\s+.*?;\s*", "", src, flags=re.MULTILINE)

    # Remove extern "C"
    src = re.sub(r"extern\s+\"C\"\s*", "", src)

    return src


def extract_functions(src: str) -> str:
    results = []
    length = len(src)
    i = 0

    while i < length:
        fn_pos = src.find("fn ", i)
        if fn_pos == -1:
            break

        # Get function name
        name_match = re.match(r"fn\s+([A-Za-z0-9_]+)", src[fn_pos:])
        if not name_match:
            i = fn_pos + 3
            continue

        fn_name = name_match.group(1)
        # Skip unwanted helper functions
        if fn_name in UNWANTED_FUNCTIONS:
            i = fn_pos + 4
            continue

        # ---------- Find nearest block comment before this fn ----------
        comment = ""
        block_start = src.rfind("/*", 0, fn_pos)
        if block_start != -1:
            block_end = src.find("*/", block_start, fn_pos)
            if block_end != -1:
                # Candidate comment block
                comment_block = src[block_start:block_end + 2]

                # Optional: only keep if it looks like a spec (contains 'precondition' or 'postcondition')
                if "precondition" in comment_block or "postcondition" in comment_block:
                    comment = comment_block.strip() + "\n"

        # ---------- Extract function body via brace matching ----------
        brace_pos = src.find("{", fn_pos)
        if brace_pos == -1:
            # malformed, bail out of loop
            break

        depth = 0
        end = brace_pos
        while end < length:
            ch = src[end]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end += 1
                    break
            end += 1

        fn_body = src[fn_pos:end].strip()

        # Combine (maybe empty) comment with function
        if comment:
            results.append(comment + fn_body)
        else:
            results.append(fn_body)

        i = end

    return "\n\n\n".join(results)


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
