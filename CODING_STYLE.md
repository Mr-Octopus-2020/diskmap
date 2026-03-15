# Coding Style for Diskmap

To keep the codebase clean and maintainable, we follow standard Rust idioms and conventions.

## 🛠️ Tooling

### Rustfmt
We use `rustfmt` to ensure consistent code formatting. Please run it before submitting any changes:
```bash
cargo fmt
```
If you have specific preferences, check [.rustfmt.toml](.rustfmt.toml).

### Clippy
We use `clippy` for linting and catching common mistakes. Ensure your code is clippy-clean:
```bash
cargo clippy -- -D warnings
```

## 🧪 Testing

We value tests! Standard unit tests should be placed in a `tests` module at the bottom of your source code file.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
```

### Doc-tests
For public APIs, we prefer documentation examples that also serve as tests:
```rust
/// Adds two integers.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The sum of `a` and `b`.
///
/// # Examples
///
/// ```
/// let result = diskmap::add(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 📏 Naming Conventions

We strictly follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html):

| Item | Convention | Example |
|------|------------|---------|
| Functions / Methods | `snake_case` | `calculate_usage()` |
| Variables | `snake_case` | `file_path` |
| Modules | `snake_case` | `visualizer` |
| Structs / Enums / Traits | `PascalCase` | `DiskMap` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_DEPTH` |
| Lifetimes | Short lowercase | `'a`, `'de` |

## 📝 Documentation

*   **Public APIs**: Must have doc comments (`///`).
*   **Complex Logic**: Use inner doc comments (`//`) to explain *why* something is done, not just *what* is being done.
*   **Check Docs**: Run `cargo doc --open` to see how your documentation looks.

## 🚀 Commit Messages

We encourage following the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat: add radial chart`, `fix: handle empty directory`).

## 💡 Final Note

Coding styles are meant to help, not hinder. If you think a rule should be changed or added, let's discuss it in an issue!