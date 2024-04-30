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

	// M2 Max: Average 0.87 seconds
	func test_json_roundtrip_no_arc() throws {
		let (sut, _) = vector
		var decoded: SUT?
		var encoded: Data?
		measure {
			encoded = profileToJsonBytes(profile: sut)
			decoded =  try! newProfileFromJsonBytes(jsonBytes: encoded!)
		}
		XCTAssertEqual(decoded, sut)
	}
	
	// M2 Max: Average 0.049
	func test_json_roundtrip_arc_in_arc_out__arc_in_arc_out() throws {
		let (sut, json) = vector
		let profileInContainer = JsonContainerProfile(profile: sut)
		var decoded: JsonContainerProfile?
		var encoded: JsonContainerBytes?
		measure {
			encoded = profileToJsonBytesArcInArcOut(profile: profileInContainer)
			decoded = try? newProfileFromJsonBytesArcInArcOut(json: encoded!)
		}
		XCTAssertEqual(encoded?.bytes(), json)
		XCTAssertEqual(decoded?.profile(), sut)
	}
	
	// M2 Max: Average 0.247
	func test_json_roundtrip_not_arc_in_arc_out__not_arc_in_arc_out() throws {
		let (sut, json) = vector
		var decoded: JsonContainerProfile?
		var encoded: JsonContainerBytes?
		measure {
			encoded = profileToJsonBytesNotArcInArcOut(profile: sut)
			decoded = try? newProfileFromJsonBytesNotArcInArcOut(json: json)
		}
		XCTAssertEqual(encoded?.bytes(), json)
		XCTAssertEqual(decoded?.profile(), sut)
	}
	
	// M2 Max: Average 0.88
	func test_json_roundtrip_not_arc_in_not_arc_out__not_arc_in_not_arc_out() throws {
		let (sut, json) = vector
		var encoded: Data?
		var decoded: SUT?
		measure {
			encoded = profileToJsonBytes(profile: sut)
			decoded = try? newProfileFromJsonBytes(jsonBytes: json)
		}
		XCTAssertEqual(encoded, json)
		XCTAssertEqual(decoded, sut)
	}
	
	// M2 Max: Average 0.058
	func test__TO_json_only__arc_in_arc_out() throws {
		let sut = vector.model
		measure {
			let _ = profileToJsonBytesArcInArcOut(profile: .init(profile: sut))
		}
	}
	
	// M2 Max: Average 0.63
	func test__TO_json_only__arc_in_not_arc_out() throws {
		let sut = vector.model
		measure {
			let _ = profileToJsonBytesArcInNotArcOut(profile: .init(profile: sut))
		}
	}
	
	// M2 Max: Average 0.057
	// 💡 CONCLUSION: The performance gain is to use Arc on result FROM Rust
	func test__TO_json_only__not_arc_in_arc_out() throws {
		let sut = vector.model
		measure {
			let _ = profileToJsonBytesNotArcInArcOut(profile: sut)
		}
	}
	
	
	// M2 Max: Average 0.195
	func test__FROM_json_only__arc_in_arc_out() throws {
		let json = vector.json
		measure {
			let _ = try! newProfileFromJsonBytesArcInArcOut(json: .init(bytes: json))
		}
	}
	
	// M2 Max: Average 0.259
	func test__FROM_json_only__arc_in_not_arc_out() throws {
		let json = vector.json
		measure {
			let _ = try! newProfileFromJsonBytesArcInNotArcOut(json: .init(bytes: json))
		}
	}
	
	// M2 Max: Average 0.199
	// 💡 CONCLUSION: The performance gain is to use Arc on result FROM Rust
	func test__FROM_json_only__not_arc_in_arc_out() throws {
		let json = vector.json
		measure {
			let _ = try! newProfileFromJsonBytesNotArcInArcOut(json: json)
		}
	}
	
	lazy var vector: (model: Profile, json: Data) = {
		try! jsonFixture(
			as: SUT.self,
			file: "huge_profile_1000_accounts",
			decode: { try Profile(jsonData: $0) }
		)
	}()
}
