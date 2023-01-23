import Foundation

@main
public struct ReadFile {
  public static func main() {
  // public static func main() throws {

    // To handle functions which can throw errors, one uses the "try" keyword.
    // Try expressions can be handled in two ways:
    // One can either use a do-catch block
    // or mark the function signature of the caller (this method) 
    // with the keyword "throws" (see above).
    // Swift forces one to handle errors.
    // Try to modify the code and see how the compiler reacts.
    // Spoiler: the program will not compile if a possible error is not handled.
    // Try to do the following:
    // 1. Remove the try keyword, compile the program
    // 2. Put it back and remove the do-catch block, compile the program
    // 3. Change the method signature to throws (see above),
    //    remove the do-catch block and compile the program
    do {
      try printFileLineByLine(path: "../README.md")
    } catch {
      // Print the error
      print("Error: \(error)")
    }

    // This example shows only one way to handle errors.
    // The main takeaway is that compared to other languages,
    // Swift forces one to handle errors at compile time.
    // This increases the reliability of the code and adds safety.
  }

  public static func printFileLineByLine(path: String) throws {
    // Try to remove the try keyword and see what happens
    // The program will not compile if a possible error is not handled
    
    // Read file contents into a string
    let fileURL = URL(fileURLWithPath: path)
    let fileContents = try String(contentsOf: fileURL, encoding: .utf8)
    
    // Separate the string into lines
    let lines = fileContents.components(separatedBy: "\n")
    // Print each line
    for line in lines {
      print(line)
    }
  }
}
