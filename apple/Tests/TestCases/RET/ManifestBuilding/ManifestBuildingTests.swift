import RegexBuilder

final class ManifestBuildingTests: XCTestCase {
	
	func test_manifest_for_faucet_with_lock_fee() {
		
		let manifest = manifestForFaucet(
			includeLockFeeInstruction: true,
			addressOfReceivingAccount: AccountAddress.sample
		)
		
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
		XCTAssert(manifest.instructionsString.contains(AccountAddress.sample.description))
		XCTAssert(manifest.instructionsString.contains("lock_fee"))
	}
	
	func test_manifest_for_faucet_without_lock_fee() {
		
		let manifest = manifestForFaucet(
			includeLockFeeInstruction: false,
			addressOfReceivingAccount: AccountAddress.sampleOther
		)
		
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
		XCTAssert(manifest.instructionsString.contains(AccountAddress.sampleOther.description))
		XCTAssertFalse(manifest.instructionsString.contains("lock_fee"))
	}
	
    func test_manifest_set_owner_keys_hashes() {
        func doTest(_ address: AddressOfAccountOrPersona, keyHashes: [PublicKeyHash]) {
            let manifest = manifestSetOwnerKeysHashes(addressOfAccountOrPersona: address, ownerKeyHashes: keyHashes)
            XCTAssert(manifest.instructionsString.contains(address.description))
            XCTAssert(manifest.instructionsString.contains("SET_METADATA"))
            XCTAssert(manifest.instructionsString.contains("owner_keys"))
        }
        
        AddressOfAccountOrPersona.allCases.forEach {
            doTest($0, keyHashes: [.sample])
            doTest($0, keyHashes: [.sample, .sampleOther])
            doTest($0, keyHashes: [.sampleOther])
        }
    }
    
    func test_create_single_fungible_token_with_metadata() {
        func doTest(_ accountAddress: AccountAddress) {
            let metadata = TokenDefinitionMetadata(
                name: UUID().uuidString,
                description: UUID().uuidString,
                symbol: UUID().uuidString,
                iconUrl: "https://example.com",
                tags: ["swift test"]
            )
            let initialSupply: Decimal192 = .pi
            let manifest = manifestCreateFungibleTokenWithMetadata(
                addressOfOwner: accountAddress,
                initialSupply: initialSupply,
                metadata: metadata
            )
            func oneOf(_ needle: String, line: UInt = #line) {
                XCTAssertEqual(manifest.instructionsString.ranges(of: needle).count, 1, line: line)
            }
            func oneIn<P: CustomStringConvertible>(metadata keyPath: KeyPath<TokenDefinitionMetadata, P>, line: UInt = #line) {
                let property = metadata[keyPath: keyPath]
                oneOf(property.description, line: line)
            }
            oneIn(metadata: \.name)
            oneIn(metadata: \.description)
            oneIn(metadata: \.symbol)
            oneOf(initialSupply.formattedPlain(locale: .test))
            oneOf(accountAddress.address)
        }
        AccountAddress.allCases.forEach(doTest)
    }

    func test_create_multiple_fungible_tokens() {
        func doTest(_ accountAddress: AccountAddress) {
            let manifest = manifestCreateMultipleFungibleTokens(addressOfOwner: accountAddress)
            XCTAssertEqual(manifest.instructionsString.ranges(of: "symbol").count, 25)
        }
        [
            AccountAddress.sampleStokenet,
            AccountAddress.sampleStokenetOther
        ].forEach(doTest)
    }
    
	func test_manifest_marking_account_as_dapp_definition_type() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestMarkingAccountAsDappDefinitionType(accountAddress: accountAddress)
			XCTAssert(manifest.instructionsString.contains(accountAddress.description))
			XCTAssert(manifest.instructionsString.contains("SET_METADATA"))
			XCTAssert(manifest.instructionsString.contains("dapp definition"))
		}
		AccountAddress.allCases.forEach(doTest)
	}
	

}
