import RegexBuilder

final class ManifestBuildingTests: XCTestCase {
	
	func test_manifest_for_faucet_with_lock_fee() {
		
		let manifest = manifestForFaucet(
			includeLockFeeInstruction: true,
			addressOfReceivingAccount: AccountAddress.sample
		)
		
		XCTAssert(manifest.description.contains("CALL_METHOD"))
		XCTAssert(manifest.description.contains(AccountAddress.sample.description))
		XCTAssert(manifest.description.contains("lock_fee"))
	}
	
	func test_manifest_for_faucet_without_lock_fee() {
		
		let manifest = manifestForFaucet(
			includeLockFeeInstruction: false,
			addressOfReceivingAccount: AccountAddress.sampleOther
		)
		
		XCTAssert(manifest.description.contains("CALL_METHOD"))
		XCTAssert(manifest.description.contains(AccountAddress.sampleOther.description))
		XCTAssertFalse(manifest.description.contains("lock_fee"))
	}
	
	func test_manifest_marking_account_as_dapp_definition_type() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestMarkingAccountAsDappDefinitionType(accountAddress: accountAddress)
			XCTAssert(manifest.description.contains(accountAddress.description))
			XCTAssert(manifest.description.contains("SET_METADATA"))
			XCTAssert(manifest.description.contains("dapp definition"))
		}
		AccountAddress.allCases.forEach(doTest)
	}
	
	func test_manifest_set_owner_keys_hashes() {
		func doTest(_ address: AddressOfAccountOrPersona, keyHashes: [PublicKeyHash]) {
			let manifest = manifestSetOwnerKeysHashes(addressOfAccountOrPersona: address, ownerKeyHashes: keyHashes)
			XCTAssert(manifest.description.contains(address.description))
			XCTAssert(manifest.description.contains("SET_METADATA"))
			XCTAssert(manifest.description.contains("owner_keys"))
		}
		
		AddressOfAccountOrPersona.allCases.forEach {
			doTest($0, keyHashes: [.sample])
			doTest($0, keyHashes: [.sample, .sampleOther])
			doTest($0, keyHashes: [.sampleOther])
		}
	}
	
	func test_create_many_fungible_tokens() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestCreateMultipleFungibleTokens(addressOfOwner: accountAddress)
			XCTAssertEqual(manifest.description.ranges(of: "symbol").count, 25)
		}
		[AccountAddress.sampleStokenet, AccountAddress.sampleStokenetOther].forEach(doTest)
	}
}
