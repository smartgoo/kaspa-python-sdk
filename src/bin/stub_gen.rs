use std::fs;

use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = kaspa::stub_info()?;
    stub.generate()?;

    post_process_stub_file("kaspa.pyi");

    Ok(())
}

fn post_process_stub_file(path: &str) {
    let content = fs::read_to_string(path).unwrap();

    let content = strip_py_prefix_from_enums(content);
    let content = rename_none_enum_variant(content);

    fs::write(path, content).unwrap();
}

/// Removes the "Py" prefix from enum class names and all their references.
/// e.g., `class PyNetworkType(enum.Enum)` becomes `class NetworkType(enum.Enum)`
fn strip_py_prefix_from_enums(content: String) -> String {
    let mut enum_names: Vec<String> = Vec::new();

    for line in content.lines() {
        if let Some(start) = line.find("class Py")
            && line.contains("(enum.Enum)")
        {
            let after_class = &line[start + 6..];
            if let Some(paren_pos) = after_class.find('(') {
                let class_name = &after_class[..paren_pos];
                if class_name.starts_with("Py") {
                    enum_names.push(class_name.to_string());
                }
            }
        }
    }

    let mut result = content;
    for py_name in &enum_names {
        if let Some(stripped) = py_name.strip_prefix("Py") {
            result = result.replace(py_name, stripped);
        }
    }

    result
}

/// Renames `None = ...` to `_None = ...` for enum variants.
/// This is necessary because `None` is a reserved keyword in Python.
fn rename_none_enum_variant(content: String) -> String {
    content.replace("    None = ...", "    _None = ...")
}
