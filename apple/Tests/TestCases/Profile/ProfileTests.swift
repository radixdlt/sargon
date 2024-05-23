import CustomDump
import Foundation
@testable import Sargon
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
		XCTAssertEqual(sut.factorSources, [dfs.asGeneral])
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
		func doTest(_ sut: SUT, _ json: String) throws {
			let encoded = sut.jsonString(prettyPrint: false)
			XCTAssertEqual(encoded, json)
			let decoded = try SUT(jsonString: json)
			XCTAssertEqual(decoded, sut)
		}
		var vectors = Array(zip(SUT.sampleValues, SUT.sampleValues.map { $0.jsonString(prettyPrint: false) }))
		vectors.append((vector.model, String(data: vector.json, encoding: .utf8)!))
		try vectors.forEach(doTest)
	}
	
	// Macbook Pro M2: 0.64 seconds
	func test_performance_json_encoding_data() throws {
		let (sut, _) = vector
		measure {
			let _ = sut.jsonData()
		}
	}
	
	// Macbook Pro M2: 0.06 (10x speedup vs BagOfBytes)
	func test_performance_json_encoding_string() throws {
		let (sut, _) = vector
		measure {
			let _ = sut.jsonString(prettyPrint: false)
		}
	}
	
	// Macbook Pro M2: 0.26 seconds
	func test_performance_json_decoding_data() throws {
		let (_, jsonData) = vector
		measure {
			let _ = try! SUT(jsonData: jsonData)
		}
	}
	
	// Macbook Pro M2: 0.1 (2.5x speedup vs BagOfBytes)
	func test_performance_json_decoding_string() throws {
		let (_, _jsonData) = vector
		let jsonString = String(data: _jsonData, encoding: .utf8)!
		measure {
			let _ = try! SUT(jsonString: jsonString)
		}
	}
	

	
	lazy var vector: (model: Profile, json: Data) = {
		try! jsonFixture(
			as: SUT.self,
			file: "huge_profile_1000_accounts",
			decode: { try Profile(jsonData: $0) }
		)
	}()

    func test_check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_not_present() {
		eachSample { sut in
			XCTAssertFalse(
				SUT.checkIfProfileJsonContainsLegacyP2PLinks(contents: sut.jsonData())
			)
		}
    }

	func test_check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present() throws {
		let json = try openFile(subPath: "vector", "only_plaintext_profile_snapshot_version_100", extension: "json")
		XCTAssert(
			SUT.checkIfProfileJsonContainsLegacyP2PLinks(contents: json)
		)
	}

	func test_check_if_encrypted_profile_json_contains_legacy_p2p_links_when_empty_json() {
		XCTAssertFalse(
			SUT.checkIfEncryptedProfileJsonContainsLegacyP2PLinks(contents: Data(), password: "babylon")
		)
	}

	func test_check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present() throws {
		let json = try openFile(subPath: "vector", "profile_encrypted_by_password_of_babylon", extension: "json")
		XCTAssert(
			SUT.checkIfEncryptedProfileJsonContainsLegacyP2PLinks(contents: json, password: "babylon")
		)
	}
}
