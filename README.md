# Does it fail? - A comparison of error handling in different languages

These are small examples to showcase how different languages handle errors. The
goal is to find out how strict the language is in enforcing error handling.

## The example

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

For a deeper explanation, look at the comments in the code.

## Contributing

If you want to add a language, feel free to open a PR. Read one of the existing
implementations for reference. You should use the best-practices of the
language, but try to only use the standard library and keep it simple. Try to
explain as much as possible in the comments but do not use field specific
jargon. It sould be understandable for people who are not familiar with the
language in which the example is written.
