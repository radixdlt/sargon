import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class MnemonicWithPassphraseTests: Test<MnemonicWithPassphrase> {
	
	func test_not_codable_but_lower_level_json_methods_json_data_roundtrip() throws{
		let sut = SUT.sample
		let json = sut.jsonData()
		try XCTAssertEqual(
			SUT(jsonData: json),
			sut
		)
	}
	
	func test_codable() throws {
		let raw = """
		{
			"mnemonic": "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate",
			"passphrase": "radix"
		}
		""".data(using: .utf8)!
		
		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)
		
		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}
	
	func test_derive_public_keys() {
		XCTAssertEqual(SUT.sample.derivePublicKeys(paths: [DerivationPath.sample]), [HierarchicalDeterministicPublicKey.sample])
	}
    
    func test_validate() {
        XCTAssertTrue(SUT.sample.validate(publicKeys: [HierarchicalDeterministicPublicKey.sample]))
        XCTAssertFalse(SUT.sampleOther.validate(publicKeys: [HierarchicalDeterministicPublicKey.sample]))
    }
	
	func test_sign_is_valid() {
		let sut = SUT.sample
		let path = DerivationPath.sample
		let publicKey = sut.derivePublicKey(path: path)
		let msg = Hash.sample
		let signature = sut.sign(hash: msg, path: path)
		XCTAssertTrue(publicKey.isValidSignature(signature, for: msg))
	}
}