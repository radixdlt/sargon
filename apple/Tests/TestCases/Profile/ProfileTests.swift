import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class ProfileTests: Test<Profile> {
	func test_description_and_debug() {
		XCTAssertGreaterThan(SUT.sample.debugDescription, SUT.sample.description)
	}

	func test_profile_description_equals() {
		XCTAssertNoDifference(SUT.sample.description, SUT.sample.description)
	}

	func test_debug_description_equals() {
		XCTAssertNoDifference(SUT.sample.debugDescription, SUT.sample.debugDescription)
		XCTAssertNoDifference(
			SUT.sampleOther.debugDescription, SUT.sampleOther.debugDescription
		)
	}

	func test_id_is_header_id() {
		XCTAssertNoDifference(SUT.sample.id, SUT.sample.header.id)
	}

	@available(*, deprecated)
	func test_encryption_roundtrip() throws {
		let password = "ultra secret"
		let sut = SUT.sample
		let encrypted = sut.encrypt(
			password: password
		)
		let decrypted = try Profile(
			encrypted: encrypted,
			decryptionPassword: password
		)
		XCTAssertNoDifference(decrypted, sut)
	}

	func test_encryption_roundtrip_string() throws {
		let password = "ultra secret"
		let sut = SUT.sample
		let encryptedString = sut.encryptedJsonString(
			password: password
		)
		let decrypted = try Profile(
			encryptedProfileJSONString: encryptedString,
			decryptionPassword: password
		)
		XCTAssertNoDifference(decrypted, sut)
	}

	func test_init_with_header_and_dfs() {
		let header = Header.sampleOther
		let dfs = DeviceFactorSource.sampleOther
		let sut = SUT(header: header, deviceFactorSource: dfs)
		XCTAssertEqual(sut.header, header)
		XCTAssertEqual(sut.appPreferences, .default)
		XCTAssertEqual(sut.networks, [])
		XCTAssertEqual(sut.factorSources, [dfs.asGeneral])
	}

	func test_analyze_file_not_profile() {
		XCTAssertEqual(SUT.analyzeContents(data: Data()), .notProfile)
	}

	func test_analyze_file_profile() {
		func doTest(_ sut: SUT, _ jsonString: String) {
			XCTAssertEqual(
				SUT.analyzeContents(of: jsonString),
				.plaintextProfile(sut)
			)
		}
		var vectors = Array(zip(SUT.sampleValues, SUT.sampleValues.map { $0.toJSONString() }))
		vectors.append(vector)

		vectors.forEach(doTest)
	}

	func test_analyze_file_encrypted_profile() {
		var vectors = SUT.sampleValues
		vectors.append(vector.model)

		let passwords = ["Mellon", "open sesame", "REINDEER FLOTILLA", "swordfish"]
		func doTest(_ index: Int, _ sut: SUT) {
			let password = passwords[index % passwords.count]
			let encrypted = sut.encryptedJsonString(password: password)
			XCTAssertEqual(
				SUT.analyzeContents(of: encrypted),
				.encryptedProfile
			)
		}

		vectors.enumerated().forEach(doTest)
	}

	func test_encrypted_profile_contents() {
		let jsonString = SUT.sample.encryptedJsonString(password: "open sesame")
		XCTAssertTrue(jsonString.contains("encryptionScheme"))
		XCTAssertTrue(jsonString.contains("keyDerivationScheme"))
		XCTAssertTrue(jsonString.contains("encryptedSnapshot"))
		XCTAssertTrue(jsonString.contains("version"))
	}

	@available(*, deprecated)
	func test_json_roundtrip() throws {
		func doTest(_ sut: SUT, _ json: String) throws {
			let encoded = sut.toJSONString(prettyPrinted: false)
			XCTAssertEqual(encoded, json)
			let decoded = try SUT(jsonString: json)
			XCTAssertEqual(decoded, sut)
			let decodedFromData = try SUT(jsonData: sut.jsonData())
			XCTAssertEqual(decodedFromData, sut)
		}
		let vectors = Array(zip(SUT.sampleValues, SUT.sampleValues.map { $0.toJSONString(prettyPrinted: false) }))
		// vectors.append(vector)
		try vectors.forEach(doTest)
	}

	/// Macbook Pro M2: 0.06 (10x speedup vs BagOfBytes)
	func test_performance_json_encoding_string() {
		let (sut, _) = vector
		measure {
			_ = sut.toJSONString()
		}
	}

	/// Macbook Pro M2: 0.1 (2.5x speedup vs BagOfBytes)
	func test_performance_json_decoding_string() {
		let (_, jsonString) = vector
		measure {
			_ = try! SUT(jsonString: jsonString)
		}
	}

	lazy var vector: (model: Profile, jsonString: String) = try! jsonString(
		as: SUT.self,
		file: "huge_profile_1000_accounts",
		decode: { try Profile(jsonString: $0) }
	)

	func test_check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_not_present() {
		eachSample { sut in
			XCTAssertFalse(
				SUT.checkIfProfileJsonStringContainsLegacyP2PLinks(jsonString: sut.toJSONString())
			)
		}
	}

	@available(*, deprecated)
	func test_check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present() throws {
		let json = try jsonData(file: "only_plaintext_profile_snapshot_version_100")
		XCTAssert(
			SUT.checkIfProfileJsonContainsLegacyP2PLinks(contents: json)
		)
	}

	func test_check_if_encrypted_profile_json_contains_legacy_p2p_links_when_empty_json() {
		XCTAssertFalse(
			SUT.checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks(jsonString: "", password: "babylon")
		)
	}

	@available(*, deprecated)
	func test_check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present() throws {
		let json = try jsonData(file: "profile_encrypted_by_password_of_babylon")
		XCTAssert(
			SUT.checkIfEncryptedProfileJsonContainsLegacyP2PLinks(contents: json, password: "babylon")
		)
	}

	func test_check_if_encrypted_profile_json_string_contains_legacy_p2p_links_when_p2p_links_are_present() throws {
		let json = try jsonData(file: "profile_encrypted_by_password_of_babylon")
		let jsonString = try XCTUnwrap(String(data: json, encoding: .utf8))
		XCTAssert(
			SUT.checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks(jsonString: jsonString, password: "babylon")
		)
	}
}
