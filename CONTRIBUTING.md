# Contributing to yew-date-range

Thank you for your interest in contributing. This document outlines the coding standards and conventions that **must** be followed for all contributions.

## Source Code Rules

All source code in this project follows a strict set of rules. These rules are **non-negotiable** and will be enforced during code review.

### RULE 1: One Type Per File

Never create two types (`struct`, `enum`, `trait`) in the same file. Each type gets its own file named in `snake_case`.

**Exception:** A type with a generic variant can share a file (e.g., a type alias alongside its source type).

### RULE 2: Directory Structure

- Use `snake_case` for all directory and file names.
- Organize by domain/feature.
- Create a `mod.rs` in each directory.

### RULE 3: File Naming

| Type | File Name |
|------|-----------|
| Struct `RangeSelection` | `range_selection.rs` |
| Enum `CalendarDirection` | `calendar_direction.rs` |
| Struct `DateHelper` | `date_helper.rs` |

### RULE 4: Visibility

Use the minimum visibility needed:

- Default: private (no keyword)
- `pub(crate)`: when used in other modules but not in the public API
- `pub`: only when part of the public API

### RULE 5: Module Documentation

Every `mod` declaration in `mod.rs` must have its own `///` doc comment directly above it:

```rust
/// Brief description of this module.
///
/// Expanded description of what this module does.
pub mod my_module;
```

### RULE 6: Type Documentation

Every `struct`, `enum`, and `trait` must have a `///` doc comment. Every field must have its own `///` doc comment. Separate fields with blank lines for readability.

### RULE 7: Method Documentation

Every public method must have a clear `///` doc comment covering:

- Brief description
- `# Parameters` section (when parameters exist)
- `# Returns` section (when a value is returned)
- `# Errors` section (when errors can occur)

### RULE 8: Inline Comments

Add a comment before **every** logical step inside method bodies using imperative mood:

```rust
pub fn process(&mut self) {
    // Validate the input parameters.
    if !self.is_valid() { return; }

    // Apply the transformation.
    self.transform();

    // Return the processed result.
    Ok(self.result())
}
```

### RULE 9: Import Organization

Organize imports in this exact order with blank lines between groups:

```rust
// 1. Standard library
use std::collections::HashMap;

// 2. External crates (alphabetically)
use time::{Date, Month};

// 3. Internal crate
use crate::models::range_selection::RangeSelection;
```

**Never** put `use` statements inside functions. Always at the top of the file.

**Always** import with `use crate::` instead of `use super::` or `use prelude::` (exception: `prelude.rs` files use `super::`).

### RULE 9.1: Prohibited Comment Styles

Never use separator comments with equal signs or dashes:

```rust
// ============================================  ← PROHIBITED
// --------------------------------------------  ← PROHIBITED
// --- Section Name ---                         ← PROHIBITED
```

### RULE 10: No Types in mod.rs

`mod.rs` files must only contain module declarations and documentation comments. All types must be in their own dedicated files.

### RULE 11: Prelude Module for Re-exports

All `pub use` statements must be placed in a `prelude` module, **not** in `mod.rs` files.

Every `prelude.rs` file must include a `//!` module-level doc comment explaining what is re-exported.

### RULE 12: No Full Path Type References

Never call types using their full path inline. Always import types at the top of the file.

### RULE 14: No Tests in Source Files

Never add `#[cfg(test)]` modules inside source files in `src/`. All tests must be placed in the `tests/` directory.

### Test Documentation

Every test file must have a `//!` module-level doc comment. Every `#[test]` function must have a `///` doc comment starting with "Validates that...":

```rust
//! Tests for date helper utility functionality.

/// Validates that days_in_month returns the correct day count for various months.
#[test]
fn test_days_in_month() {
    // ...
}
```

### No unwrap() or expect()

Never use `unwrap()` or `expect()` anywhere in the codebase, including test code. Use safe alternatives like `unwrap_or`, `match`, or the `?` operator.

## Pull Request Process

1. **Fork** the repository and create a feature branch from `master`.
2. **Follow all rules** listed above without exception.
3. **Add tests** for any new functionality in the `tests/` directory.
4. **Run the full test suite** before submitting:
   ```bash
   cargo test --package yew-date-range-core --target x86_64-unknown-linux-gnu
   ```
5. **Run formatting and linting**:
   ```bash
   cargo fmt --all
   cargo clippy --package yew-date-range-core --all-targets -- -D warnings
   ```
6. **Open a pull request** with a clear description of the changes.

## Architecture

The library follows a two-crate design:

- **`yew-date-range-core`** — Headless core with models and utilities. No Yew dependency.
- **`yew-date-range`** — Yew UI components and styles. Depends on the core crate.

Utility logic uses **structs with static functions** (e.g., `DateHelper`, `RangeHelper`, `LocaleHelper`) rather than orphan free functions.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
