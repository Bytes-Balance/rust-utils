# Clickable Hyperlinks in Bash and Rust

This repository contains code examples that demonstrate how to create clickable hyperlinks in both Bash and Rust. The examples use ANSI escape codes for Bash and Rust's `Display` trait to format hyperlinks that can be used in terminal environments.

## Features

- **Bash Hyperlinks**: Learn how to create clickable hyperlinks in a Bash terminal using ANSI escape codes.
- **Rust Hyperlinks**: Explore how to define a `HyperLink` struct in Rust and implement the `Display` trait to generate clickable hyperlinks.


## Bash Example

To test the Bash hyperlink, simply run the following command in your terminal:

```bash
echo -e '\033]8;;https://github.com/sponsors/NickLarsenNZ\033\\Kudos to Nick Larsen for the code! Follow him on GitHub Sponsors!\033]8;;\033\\'
```

## Rust Example

1. Clone the repository:

   ```bash
   git clone https://github.com/Bytes-Balance/rust-utils.git
   ```

2. Navigate to the directory and run the program:

   ```bash
   cargo run --example hyperlink
   ```

3. The Rust program will print a clickable hyperlink in the terminal:

   ```plaintext
   Kudos to Nick Larsen for the code! Follow him on GitHub Sponsors!
   ```

## Acknowledgments

- [Nick Larsen](https://github.com/sponsors/NickLarsenNZ) for the inspiration and examples used in this project.
