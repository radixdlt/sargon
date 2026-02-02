import Foundation
import SargonUniFFI
import XCTest

final class SecurityProblemTests: TestCase {
	typealias SUT = SecurityProblem

	func testKind() {
		var sut = SUT.problem3(addresses: .init(accounts: [], hiddenAccounts: [], personas: [], hiddenPersonas: []))
		XCTAssertEqual(sut.kind, .securityFactors)

		sut = SUT.problem5
		XCTAssertEqual(sut.kind, .configurationBackup)

		sut = SUT.problem6
		XCTAssertEqual(sut.kind, .configurationBackup)

		sut = SUT.problem7
		XCTAssertEqual(sut.kind, .configurationBackup)

		sut = .problem9(addresses: .init(accounts: [], hiddenAccounts: [], personas: [], hiddenPersonas: []))
		XCTAssertEqual(sut.kind, .securityFactors)
	}

	func testId() {
		var sut = SUT.problem3(addresses: .init(accounts: [], hiddenAccounts: [], personas: [], hiddenPersonas: []))
		XCTAssertEqual(sut.id, 3)

		sut = SUT.problem5
		XCTAssertEqual(sut.id, 5)

		sut = SUT.problem6
		XCTAssertEqual(sut.id, 6)

		sut = SUT.problem7
		XCTAssertEqual(sut.id, 7)

		sut = .problem9(addresses: .init(accounts: [], hiddenAccounts: [], personas: [], hiddenPersonas: []))
		XCTAssertEqual(sut.id, 9)
	}
}
