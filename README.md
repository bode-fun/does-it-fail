# Does it fail? - A comparison of error handling in different languages

These are small examples to showcase how different languages handle errors. The
goal is to find out how strict the language is in enforcing error handling. This
is not meant to be a scientific in any way, shape or form. It is just me,
experimenting with different languages.

## The example

1. Create a program which reads it's local README.md file and prints it line by
   line.
2. See how the langugae enfoces you to handle possible errors.

### How to run the examples

1. Install a container runtime like
   [docker via Rancher Desktop](https://rancherdesktop.io/) (my favourite),
   [podman](https://podman.io/) or
   [colima](https://github.com/abiosoft/colima/).
   1. If you want to use something other than docker, you will have to change
      the `justfile` to use your container runtime.
2. Install [just](https://just.systems/).
3. Run `just run <folder>` to run the example in a container.

### How to run the examples on your machine

- Take a look at the description in the `README.md` of the language you want to
  run.

## Results

| Language              | Enforced error handling |
| --------------------- | ----------------------- |
| Rust                  | ✅                      |
| TypeScript (via Deno) | ❌                      |
| Swift                 | ✅                      |

For a deeper explanation, look at the comments in the code.

## Contributing

If you want to add a language, feel free to open a PR. Read one of the existing
implementations for reference. You should use the best-practices of the
language, but try to only use the standard library and keep it simple. Try to
explain as much as possible in the comments but do not use field specific
jargon. It sould be understandable for people who are not familiar with the
language in which the example is written. Don't be afraid to PR your work in
progress. I will be happy to help you.
