import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class TestCase: XCTestCase {
	override func setUp() {
		self.continueAfterFailure = false
	}
	
	func openFile(
		subPath: String,
		_ fileName: String,
		extension fileExtension: String
	) throws -> Data {
		let testsDirectory: String = URL(fileURLWithPath: "\(#file)").pathComponents.dropLast(4).joined(separator: "/")
		
		let fileURL = try XCTUnwrap(URL(fileURLWithPath: "\(testsDirectory)/fixtures/\(subPath)/\(fileName).\(fileExtension)"))
		
		return try Data(contentsOf: fileURL)
	  
	}
	
	func jsonFixture<T: Decodable>(
		_ fileName: String,
		decode: (Data) throws -> T = { try JSONDecoder().decode(T.self, from: $0) }
	) throws -> T {
		let json = try openFile(subPath: "vector", fileName, extension: "json")
		return try decode(json)
	}
	
}

class Test<SUT_: SargonModel>: TestCase {
	typealias SUT = SUT_
	
	func eachSample(
		_ test: (SUT) throws -> Void
	) rethrows {
		try SUT.sampleValues.forEach(test)
	}
	
	func test_equality() throws {
		XCTAssertNoDifference(SUT.sample, SUT.sample)
	}

	func test_inequality() throws {
		XCTAssertNotEqual(SUT.sample, SUT.sampleOther)
	}
	
	func test_hashable() {
		XCTAssertNoDifference(Set([SUT.sample, SUT.sample]).count, 1)
		XCTAssertNoDifference(Set([SUT.sampleOther, SUT.sampleOther]).count, 1)
		
		var set = Set<SUT>()
		SUT.sampleValues.forEach { set.insert($0) }
		SUT.sampleValues.forEach { set.insert($0) } // duplicates removed.
		XCTAssertGreaterThanOrEqual(set.count, 2)
	}

	func test_custom_string_convertible() throws {
		guard
			let sample = SUT.sample as? CustomStringConvertible,
			let sampleOther = SUT.sample as? CustomStringConvertible
		else {
			return
		}
		XCTAssertNoDifference(sample.description, sample.description)
		XCTAssertNoDifference(sampleOther.description, sampleOther.description)
	}
}

extension Test where SUT: Codable {
	func doTestCodableRoundtrip(_ sut: SUT) throws {
		let jsonEncoder = JSONEncoder()
		let jsonDecoder = JSONDecoder()
		let data = try jsonEncoder.encode(sut)
		let decoded = try jsonDecoder.decode(SUT.self, from: data)
		XCTAssertEqual(decoded, sut)
	}
}
