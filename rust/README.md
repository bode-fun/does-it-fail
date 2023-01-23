# Read test in Rust 🦀

## Run the program

### In a container (recommended)

1. Install [docker via Rancher Desktop](https://rancherdesktop.io/),
   [podman](https://podman.io/) or [colima](https://github.com/abiosoft/colima)
   1. Rancher Desktop is an excelent alternative do "Docker Desktop". This is my
      preferred way to run docker on macOS and Windows and Linux.
   2. If you want to use something other than docker, you will have to change
      the `justfile` to use your container runtime.
2. Install [just](https://just.systems/)
3. Go into the root directory of this repository
4. Run `just run rust` to build run the program in a container
5. Fiddle with the code and see how it affects the error handling

### On your machine

3. Install [rust](https://www.rust-lang.org/learn/get-started)
4. Run `cargo run` in this directory
5. To run in release mode, run `cargo run --release`
6. Fiddle with the code and see how it affects the error handling
