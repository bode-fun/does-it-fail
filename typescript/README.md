# Read test in TypeScript with Deno 🦕

## Run the program

### In a container (recommended)

1. Install [docker via Rancher Desktop](https://rancherdesktop.io/),
   [podman](https://podman.io/) or [colima](https://github.com/abiosoft/colima/)
   1. Rancher Desktop is an excelent alternative do "Docker Desktop". This is my
      preferred way to run docker on macOS and Windows and Linux
   2. If you want to use something other than docker, you will have to change
      the `justfile` to use your container runtime.
2. Install [just](https://just.systems/)
3. Go into the root directory of this repository
4. Run `just run typescript` to build run the program in a container
5. Fiddle with the code and see how it affects the error handling

### On your machine

1. Install [deno](https://deno.land/manual/getting_started/installation)
2. Run `deno run main.ts` in this directory
3. Fiddle with the code and see how it affects the error handling
4. Check the code with `deno lint main.ts`
