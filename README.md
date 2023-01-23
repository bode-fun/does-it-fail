# Comparison of error handling in different languages

This is a small test I wrote for comparing runtime-error handling between
different languages.

## The test

1. Create a program which reads this file and prints it line by line.
2. See how the langugae enfoces you to handle possible errors.

## Implemented languages

- [x] Rust
- [x] TypeScript (with Deno)
- [x] Swift

## Results

| Language   | Enforced error handling |
| ---------- | ----------------------- |
| Rust       | ✅                      |
| TypeScript | ❌                      |
| Swift      | ✅                      |

## Contributing

If you want to add a language, feel free to open a PR. Read one of the existing
implementations for reference. You should use the best-practices of the
language, but try to only use the standard library and keep it simple.
