import RegexBuilder

final class ManifestBuildingTests: TestCase {
	
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
                XCTAssertEqual(manifest.description.ranges(of: needle).count, 1, line: line)
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
	
	func test_create_single_fungible_token() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestCreateFungibleToken(addressOfOwner: accountAddress)
			XCTAssertEqual(manifest.description.ranges(of: "Stella").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "STAR").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "The brightest component in the Radix ecosystem.").count, 1)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.allCases.forEach(doTest)
	}

    func test_create_multiple_fungible_tokens() {
        func doTest(_ accountAddress: AccountAddress) {
            let manifest = manifestCreateMultipleFungibleTokens(addressOfOwner: accountAddress)
            XCTAssertEqual(manifest.description.ranges(of: "symbol").count, 25)
			XCTAssert(manifest.description.contains(accountAddress.address))
        }
		
        [
            AccountAddress.sampleStokenet,
            AccountAddress.sampleStokenetOther
        ].forEach(doTest)
    }
	
	func test_create_single_nft_collection() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestCreateNonFungibleToken(addressOfOwner: accountAddress)
			XCTAssertEqual(manifest.description.ranges(of: "An amazingly innovative and rare NFT collection").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "nf-number").count, 20)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.allCases.forEach(doTest)
	}

	func test_create_multiple_nft_collection() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestCreateMultipleNonFungibleTokens(addressOfOwner: accountAddress)
			let collections = 15
			let nftsPerCollection = 10
			XCTAssertEqual(manifest.description.ranges(of: "An amazingly innovative and rare NFT collection").count, collections)
			XCTAssertEqual(manifest.description.ranges(of: "nf-number").count, collections * nftsPerCollection)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.allCases.forEach(doTest)
	}
	
	func test_stakes_claim() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = manifestStakesClaim(accountAddress: accountAddress, stakeClaims: StakeClaim.allCases)
			XCTAssertEqual(manifest.description.ranges(of: StakeClaim.sample.validatorAddress.mapTo(networkID: accountAddress.networkID).address).count, 1)
			XCTAssertEqual(manifest.description.ranges(of: accountAddress.xrd.address).count, 2)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		
		AccountAddress.allCases.forEach(doTest)
	}
}
