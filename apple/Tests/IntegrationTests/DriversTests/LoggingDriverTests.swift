import CustomDump
import Foundation
import os
@testable import Sargon
import SargonUniFFI
import XCTest

class LoggingDriverTests: DriverTest<Log> {
	func test() {
		let sut = SUT.shared
		let levels = LogLevel.allCases
		for level in levels {
			let msg = "Swift Unit test \(#file) \(#line)"
			sut.log(level: level, msg: msg)
			sut.swiftLogger.log(level: .init(sargonLogLevel: level), "\(msg)")
		}
	}

	func test_setRustLogLevel() {
		for level in LogFilter.allCases {
			setRustLogLevel(level)
			XCTAssertEqual(getRustLogLevel(), level)
		}
	}

	func test_os_log_type_from_loglevel() {
		func doTest(_ from: Sargon.LogLevel, _ expected: OSLogType) {
			XCTAssertEqual(OSLogType(sargonLogLevel: from), expected)
		}
		doTest(.error, .fault)
		doTest(.warn, .error)
		doTest(.info, .info)
		doTest(.debug, .default)
		doTest(.trace, .debug)
	}

	func test_os_log_type_from_filter() {
		func doTest(_ from: Sargon.LogFilter, _ expected: OSLogType) {
			XCTAssertEqual(OSLogType(sargonFilterLevel: from), expected)
		}
		doTest(.off, .fault) // yes inaccurate, but not too important, can fix later.
		doTest(.error, .fault)
		doTest(.warn, .error)
		doTest(.info, .info)
		doTest(.debug, .default)
		doTest(.trace, .debug)
	}
}
