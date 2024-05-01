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
		XCTAssertEqual(sut.factorSources.elements, [dfs.asGeneral])
	}

	func test_analyze_file_not_profile() {
		XCTAssertEqual(SUT.analyzeFile(contents: Data()), .notProfile)
	}

	func test_analyze_file_profile() {
		func doTest(_ sut: SUT) {
			XCTAssertEqual(
				SUT.analyzeFile(reference: sut.profileSnapshotRef()),
				.plaintextProfile(RefProfile(inner: sut))
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
	
	func test_json_slow_correctness() throws {
		let (sut, json) = vector
		let encoded = sut.profileSnapshot()
		let decoded = try SUT(jsonBytes: json)
		XCTAssertEqual(encoded, json)
		XCTAssertEqual(decoded, sut)
	}
	
	func test_json_fast_correctness() throws {
		let (sut, json) = vector
		let refBytes = sut.profileSnapshotRef()
		try XCTAssertEqual(refBytes.take(), json)
		let decoded = try SUT(jsonBytesReference: .init(inner: json))
		XCTAssertEqual(decoded, sut)
	}
	
	// M2 Max: Average 0.6
	// OMITTED - too slow (6 seconds)
	func json_encoding_slow_performance() throws {
		let (sut, _) = vector
		measure {
			let _ = profileToJsonBytes(profile: sut)
		}
	}
	
	// M2 Max: Average 0.003 (200x as fast)
	func test_json_encoding_fast_performance() throws {
		let (sut, _) = vector
		measureMetrics([.wallClockTime], automaticallyStartMeasuring: false) {
			let reference = RefProfile(inner: sut)
			startMeasuring()
			let _ = profileToJsonBytesFastByRef(reference: reference)
		}
	}
	
	// M2 Max: Average 0.26
	// OMITTED - too slow (4 seconds)
	func json_decoding_slow_performance() throws {
		let (_, json) = vector
		measure {
			let _ = try! newProfileFromJsonBytes(jsonBytes: json)
		}
	}
	
	// M2 Max: Average 0.042 (6x as fast)
	func test_json_decoding_fast_performance() throws {
		let (_, json) = vector
		measureMetrics([.wallClockTime], automaticallyStartMeasuring: false) {
			let reference = RefBytes(inner: json)
			startMeasuring()
			let _ = try! newProfileFromJsonBytesFastByRef(reference: reference)
		}
	}

	func test_json_encoding_performance_compare_slow_vs_fast() throws {
		let (sut, _) = vector
		
		func measure<T>(_ operation: (RefProfile) throws -> T) rethrows -> Double {
			let profileRef = RefProfile(inner: sut)
			let beginTime = mach_absolute_time()
			let _ = try operation(profileRef)
			return Double(mach_absolute_time()) - Double(beginTime)
		}
		
		let slow = measure { _ in
			profileToJsonBytes(profile: sut)
		}
		
		let fast = measure {
			profileToJsonBytesFastByRef(reference: $0)
		}
		
		let speedUpFactor = slow / fast
		print("🐢 speedup \(speedUpFactor) 🐰")
		XCTAssertGreaterThan(speedUpFactor, 150) // fast by ref is more than 150 times as fast
	}
	
	func test_json_decoding_performance_compare_slow_vs_fast() throws {
		let (_, json) = vector
		
		func measure<T>(_ operation: (RefBytes) throws -> T) rethrows -> Double {
			let bytesRef = RefBytes(inner: json)
			let beginTime = mach_absolute_time()
			let _ = try operation(bytesRef)
			return Double(mach_absolute_time()) - Double(beginTime)
		}
		
		let slow = try measure { _ in
			try newProfileFromJsonBytes(jsonBytes: json)
		}
		
		let fast = try measure {
			try newProfileFromJsonBytesFastByRef(reference: $0)
		}
		
		let speedUpFactor = slow / fast
		print("🐢 speedup \(speedUpFactor) 🐰")
		XCTAssertGreaterThan(speedUpFactor, 5) // fast by ref is more than 5 times as fast
	}
	
	lazy var vector: (model: Profile, json: Data) = {
		try! jsonFixture(
			as: SUT.self,
			file: "huge_profile_1000_accounts",
			decodeWithoutDecoder: { try Profile(jsonBytesReference: .init(inner: $0)) }
		)
	}()
}

extension Profile {
	public static func analyzeFile(
		contents: some DataProtocol
	) -> ProfileFileContents {
		Self.analyzeFile(reference: .init(inner: Data(contents)))
	}
	
}
