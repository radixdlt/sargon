// Inspired by: https://gist.github.com/nschum/208b3dde43785afd439a

public func precondition(
	_ condition: @autoclosure () -> Bool,
	_ message: @autoclosure () -> String = "",
	file: StaticString = #file,
	line: UInt = #line
) {
	preconditionClosure(condition(), message(), file, line)
}

typealias Precondition = (
	_ /* condition */: Bool,
	_ /* message */: String,
	_ /* file */: StaticString,
	_ /* line */: UInt
) -> Void;

/// The actual function called by our custom `precondition`.
var preconditionClosure: Precondition = defaultPreconditionClosure
let defaultPreconditionClosure = {Swift.precondition($0, $1, file: $2, line: $3)}
