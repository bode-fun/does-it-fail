# Read test in Swift 🐦

## Run the program

### In a container

1. Install [docker via Rancher Desktop](https://rancherdesktop.io/),
   [podman](https://podman.io/) or [colima](https://github.com/abiosoft/colima/)
   1. Rancher Desktop is an excelent alternative do "Docker Desktop". This is my
      preferred way to run docker on macOS and Windows and Linux.
   2. If you want to use something other than docker, you will have to change
      the `justfile` to use your container runtime.
2. Install [just](https://just.systems/)
3. Go into the root directory of this repository
4. Run `just run swift` to build run the program in a container
5. Fiddle with the code and see how it affects the error handling

### On your machine (recommended, because the swift container image is huge)

1. Install [swift](https://www.swift.org/getting-started/)
2. Run `swift run` in this directory
3. To run in release mode, run `swift run -c release`
4. Fiddle with the code and see how it affects the error handling
