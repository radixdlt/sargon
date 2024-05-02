import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class ProfileTests: Test<Profile> {

	func test_profile_description_equals() throws {
		XCTAssertNoDifference(SUT.sample.description, SUT.sample.description)
	}

	func test_debug_description_equals() throws {
		XCTAssertNoDifference(SUT.sample.debugDescription, SUT.sample.debugDescription)
		XCTAssertNoDifference(
			SUT.sampleOther.debugDescription, SUT.sampleOther.debugDescription)
	}

	func test_id_is_header_id() {
		XCTAssertNoDifference(SUT.sample.id, SUT.sample.header.id)
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
		func doTest(_ sut: SUT, _ json: Data) {
			XCTAssertEqual(
				SUT.analyzeFile(contents: json),
				.plaintextProfile(sut)
			)
		}
		var vectors = Array(zip(SUT.sampleValues, SUT.sampleValues.map { $0.jsonData() }))
		vectors.append(vector)
		
		vectors.forEach(doTest)
	}

	func test_analyze_file_encrypted_profile() {
	
		var vectors = SUT.sampleValues
		vectors.append(vector.model)
		
		let passwords = ["Mellon", "open sesame", "REINDEER FLOTILLA", "swordfish"]
		func doTest(_ index: Int, _ sut: SUT) {
			let password = passwords[index % passwords.count]
			let encrypted = sut.encrypt(password: password)
			XCTAssertEqual(
				SUT.analyzeFile(contents: encrypted),
				.encryptedProfile
			)
		}
		
		vectors.enumerated().forEach(doTest)
	}

	func test_encrypted_profile_contents() throws {
		let encrypted = SUT.sample.encrypt(password: "open sesame")
		let jsonString = try XCTUnwrap(String(data: encrypted, encoding: .utf8))
		XCTAssertTrue(jsonString.contains("encryptionScheme"))
		XCTAssertTrue(jsonString.contains("keyDerivationScheme"))
		XCTAssertTrue(jsonString.contains("encryptedSnapshot"))
		XCTAssertTrue(jsonString.contains("version"))
	}
	
	func test_json_roundtrip() throws {
		func doTest(_ sut: SUT, _ json: Data) throws {
			let encoded = sut.profileSnapshot()
			XCTAssertEqual(encoded, json)
			let decoded = try SUT(jsonData: json)
			XCTAssertEqual(decoded, sut)
		}
		var vectors = Array(zip(SUT.sampleValues, SUT.sampleValues.map { $0.jsonData() }))
		vectors.append(vector)
		try vectors.forEach(doTest)
	}
	
	lazy var vector: (model: Profile, json: Data) = {
		try! jsonFixture(
			as: SUT.self,
			file: "huge_profile_1000_accounts",
			decode: { try Profile(jsonData: $0) }
		)
	}()

        func test_check_if_profile_json_contains_legacy_p2p_links() {
            func doTest(_ sut: SUT) {
                XCTAssertEqual(
                    SUT.checkIfProfileJsonContainsLegacyP2PLinks(contents: sut.jsonData()),
                    false
                )
            }
            SUT.sampleValues.forEach(doTest)
        }
}
