import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import SwiftyJSON
import XCTest

final class FactorSourcesTests: CollectionTest<FactorSource> {
	override class func sample() -> SUT {
		SUT.sample
	}

	override class func sampleOther() -> SUT {
		SUT.sampleOther
	}

	/// Have to omit this test... obviously... since it crashes.
	/// We can have this test implemented when swift-testing is stable to be used,
	/// and we will use "exit tests" to test it:
	/// https://forums.swift.org/t/exit-tests-death-tests-and-you/71186
	@available(*, deprecated)
	func omit_crash_if_empty() {
		var profile = Profile.sample
		profile.factorSources = [] // empty FactorSources is not allowed
		let _ = profile.jsonData() // should crash
	}

	@available(*, deprecated)
	func test_json_decoding_of_profile_fails_if_factorSources_is_empty() throws {
		var json = JSON(Profile.sample)
		json["factorSources"] = []
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}

	@available(*, deprecated)
	func test_json_decoding_of_profile_fails_if_factorSources_contains_duplicates() throws {
		var json = JSON(Profile.sample)
		json["factorSources"] = [FactorSource.sample, FactorSource.sample]
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}

	@available(*, deprecated)
	func test_json_decoding_of_profile_fails_if_factorSources_contains_duplicated_ids() throws {
		var json = JSON(Profile.sample)
		let a = FactorSource.sample
		var b = a
		b.common.addedOn = .now
		json["factorSources"] = [a, b]
		XCTAssertThrowsError(try Profile(jsonData: json.rawData()))
	}
}
