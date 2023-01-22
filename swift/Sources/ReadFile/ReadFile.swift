import Foundation

@main
public struct ReadFile {
  public static func main() {
    // A try expresion has to be wrapped in a do-catch block
    // or the function has to be marked as throws
    do {
      try printFileLineByLine(path: "../README.md")
    } catch {
      print("Error: \(error)")
    }
  }

  public static func printFileLineByLine(path: String) throws {
    // Read file contents into a string
    // Try to remove the try keyword and see what happens
    // The program will not compile if a possible error is not handled
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
