
#Rust String Utilities

## Description

`rust_string_utils` is a Rust library providing various utility functions for string manipulation.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_string_utils = "0.1.0"
```

## Usage

Here are some examples of how to use the functions provided by this library:

### Check if a string is empty

```rust
use rust_string_utils::is_empty;

let result = is_empty(String::from(""));
assert_eq!(result, true);
```

### Check if a string is blank

```rust
use rust_string_utils::is_blank;

let result = is_blank(String::from("   "));
assert_eq!(result, true);
```

### Reverse a string

```rust
use rust_string_utils::reverse;

let result = reverse(String::from("abcde"));
assert_eq!("edcba", result);
```

### Check if a string starts with a prefix

```rust
use rust_string_utils::start_with;

let result = start_with(String::from("abcde"), String::from("a"));
assert_eq!(true, result);
```

### Join characters with a delimiter

```rust
use rust_string_utils::join_char;

let result = join_char(vec!['a', 'b', 'c'], ',');
assert_eq!("a,b,c", result);
```

## License

This project is licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

# rust_string_utils
