import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DateTests: TestCase {
	func test_date() {
		// This matches `[bindings.swift.custom_types.Timestamp]`
		// inside `uniffi.toml` with the only difference that we use `$0` here but
		// the toml uses `{}`;
		let into_custom: (String) -> Date = {
			let stringToDeserialize = $0
			let formatter = ISO8601DateFormatter()
			let formatOptionMS = ISO8601DateFormatter.Options.withFractionalSeconds
			formatter.formatOptions.insert(formatOptionMS)

			func format() -> Date? {
				formatter.date(from: stringToDeserialize)
			}

			if let date = format() {
				return date
			}

			// try without fractional seconds
			formatter.formatOptions.remove(formatOptionMS)
			return format()!
		}

		// This matches `[bindings.swift.custom_types.Timestamp]`
		// inside `uniffi.toml`
		let from_custom: (Date) -> String = {
			let dateToSerialize = $0
			let formatter = ISO8601DateFormatter()
			formatter.formatOptions.insert(.withFractionalSeconds)
			return formatter.string(from: dateToSerialize)
		}

		func testIntoThenFrom(_ vector: (String, String?)) {
			let sut = vector.0
			let expected = vector.1 ?? sut

			let string = from_custom(into_custom(sut))

			XCTAssertEqual(string, expected)
		}

		func testFromThenInto(_ vector: (String, String?)) {
			let lhs = vector.0
			let rhs = vector.1 ?? lhs

			let lhsString = from_custom(into_custom(lhs))
			let rhsString = from_custom(into_custom(rhs))

			XCTAssertEqual(rhsString, rhs)

			XCTAssertEqual(
				into_custom(lhsString),
				into_custom(rhsString)
			)
		}

		let vectors = [
			("2023-12-24T17:13:56.123456Z", "2023-12-24T17:13:56.123Z"), // precision lost (which is OK)
			("2023-12-24T17:13:56.123Z", nil), // unchanged
			("2023-12-24T17:13:56Z", "2023-12-24T17:13:56.000Z"), // (000 added, which is OK)
		]
		vectors.forEach(testIntoThenFrom)
		vectors.forEach(testFromThenInto)
	}
}
