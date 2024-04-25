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
}
