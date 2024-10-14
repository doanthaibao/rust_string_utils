
# Rust String Utilities

## Description

`rust_string_utils` is a Rust library providing various utility functions for string manipulation.

## Repository
[GitHub Repository](https://github.com/doanthaibao/rust_string_utils)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_string_utils = "0.1.7"
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

### Replace a string in a string
```rust
let result = replace(String::from("hello world"), String::from("world"), String::from("Rust"));
assert_eq!(result, "hello Rust");
```

### Replace a character in a string
```rust
let result = replace_char(String::from("hello world"), 'o', 'O');
assert_eq!(result, "hellO wOrld");
```

### Count the number of occurrences of a character in a string
```rust
let count = count_matches(String::from("hello world"), 'o');
assert_eq!(count, 2);
```

### Triple a string
```rust
let result = trip(String::from("a b c "));
assert_eq!("abc", result);
```

### Swap case of a string
```rust
let result = swap_case(String::from("Hello World"));
assert_eq!("hELLO wORLD", result);
```

## License

This project is licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)


