@testable import Sargon

extension XCTestCase {
	
	func shouldPanic<T>(
		expected expectedMessage: String,
		action: () -> T
	) {
		let exp = expectation(description: "failing precondition")

		// Overwrite `precondition` with something that doesn't terminate but verifies it happened.
		preconditionClosure = { condition, message, file, line in
			if !condition {
				exp.fulfill()
				XCTAssertEqual(
					message,
					expectedMessage,
					"precondition message didn't match",
					file: file,
					line: line
				)
			}
		}

		// Call code.
		_ = action()

		// Verify precondition "failed".
		wait(for: [exp])

		// Reset precondition.
		preconditionClosure = defaultPreconditionClosure
	}
}
