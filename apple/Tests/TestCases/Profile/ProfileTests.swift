import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ProfileTests: Test<Profile> {

	func test_description_and_debug() {
		XCTAssertGreaterThan(SUT.sample.debugDescription, SUT.sample.description)
	}
	
	func test_profile_description_equals() throws {
		XCTAssertNoDifference(SUT.sample.description, SUT.sample.description)
	}

	func test_debug_description_equals() throws {
		XCTAssertNoDifference(SUT.sample.debugDescription, SUT.sample.debugDescription)
		XCTAssertNoDifference(SUT.sampleOther.debugDescription, SUT.sampleOther.debugDescription)
	}
	
	func test_id_is_header_id() {
		XCTAssertNoDifference(SUT.sample.id, SUT.sample.header.id)
	}
    
    func test_codable_roundtrip() throws {
        try SUT.sampleValues.forEach(doTestCodableRoundtrip)
    }
	
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
	
	func test_init_with_header_and_dfs() {
		let header = Header.sampleOther
		let dfs = DeviceFactorSource.sampleOther
		let sut = SUT(header: header, deviceFactorSource: dfs)
		XCTAssertEqual(sut.header, header)
		XCTAssertEqual(sut.appPreferences, .default)
		XCTAssertEqual(sut.networks, [])
		XCTAssertEqual(sut.factorSources.elements, [dfs.asGeneral])
	}
	
	func test_analyze_file_not_profile() {
		XCTAssertEqual(SUT.analyzeFile(contents: Data()), .notProfile)
	}
	
	func test_analyze_file_profile() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(
				SUT.analyzeFile(contents: sut.jsonData()),
				.plaintextProfile(sut)
			)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_analyze_file_encrypted_profile() {
		func doTest(_ sut: SUT) {
			let encrypted = sut.encrypt(password: "melon")
			XCTAssertEqual(
				SUT.analyzeFile(contents: encrypted),
				.encryptedProfile
			)
		}
		SUT.sampleValues.forEach(doTest)
	}
	
	func test_encrypted_profile_contents() throws {
		let encrypted = SUT.sample.encrypt(password: "open sesame")
		let jsonString = try XCTUnwrap(String(data: encrypted, encoding: .utf8))
		XCTAssertTrue(jsonString.contains("encryptionScheme"))
		XCTAssertTrue(jsonString.contains("keyDerivationScheme"))
		XCTAssertTrue(jsonString.contains("encryptedSnapshot"))
		XCTAssertTrue(jsonString.contains("version"))
	}
}
