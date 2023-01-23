async function printFileLineByLine(path: string) {
  // The Deno.readFile function returns a Promise (which the await keyword is used for),
  // which is a good indicator that an error can be thrown.
  // If an error is thrown, it will bubble up to the caller of this function.
  // Compeare this to the sync version of this function below.
  const file = await Deno.readFile(path);
  const fileContents = new TextDecoder().decode(file);

  const lines = fileContents.split("\n");
  for (const line of lines) {
    console.log(line);
  }
}

function printFileLineByLineSync(path: string) {
  // This implementation is the same as the async version above,
  // Instead of using Deno.readFile, it uses Deno.readFileSync,
  // which is a synchronous version of the readFile function.
  // It can potentially throw an error, but this is not obvious from the
  // function signature. The async version of  this functions returns a
  // Promise, which is always a good indicator that an error can be thrown.
  const file = Deno.readFileSync(path); // <-- We are not forced to handle this error
  const fileContents = new TextDecoder().decode(file);

  const lines = fileContents.split("\n");
  for (const line of lines) {
    console.log(line);
  }
}

if (import.meta.main) {
  // To handle errors, wrap the function call in a try-catch block.
  // A await keyword is used because the function returns a Promise.
  // Promises are a good indicator that an error can be thrown.
  // JavaScrtipt does not force you to handle errors.
  // Try to modify the code to see what happens when an error is thrown.
  // 1. Remove the try-catch block
  // 2. Remove changes the path to a non-existing file (remove the '.md')
  // 3. Change the printFileLineByLine function to printFileLineByLineSync and
  //    remove the await keyword.
  // 4. Change the code in the printFileLineByLine function
  try {
    await printFileLineByLine("../README.md");
    // printFileLineByLineSync("../README.md");
  } catch (error: unknown) {
    if (error instanceof Error) {
      console.error(`Error: ${error.message}`);
    } else {
      console.error("Unknown error");
    }
  }
}
