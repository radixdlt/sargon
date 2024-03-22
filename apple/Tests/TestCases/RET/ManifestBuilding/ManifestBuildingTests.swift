import RegexBuilder

final class ManifestBuildingTests: Test<TransactionManifest> {
	
	func test_manifest_for_faucet_with_lock_fee() {
		
        let manifest = SUT.faucet(
			includeLockFeeInstruction: true,
			addressOfReceivingAccount: AccountAddress.sample
		)
		
		XCTAssert(manifest.description.contains("CALL_METHOD"))
		XCTAssert(manifest.description.contains(AccountAddress.sample.description))
		XCTAssert(manifest.description.contains("lock_fee"))
	}

	func test_manifest_for_faucet_without_lock_fee() {
		
        let manifest = SUT.faucet(
			includeLockFeeInstruction: false,
			addressOfReceivingAccount: AccountAddress.sampleOther
		)
		
		XCTAssert(manifest.description.contains("CALL_METHOD"))
		XCTAssert(manifest.description.contains(AccountAddress.sampleOther.description))
		XCTAssertFalse(manifest.description.contains("lock_fee"))
	}
	
	
	func test_manifest_marking_account_as_dapp_definition_type() {
		func doTest(_ accountAddress: AccountAddress) {
            let manifest = SUT.markingAccountAsDappDefinitionType(accountAddress: accountAddress)
			XCTAssert(manifest.description.contains(accountAddress.description))
			XCTAssert(manifest.description.contains("SET_METADATA"))
			XCTAssert(manifest.description.contains("dapp definition"))
		}
		AccountAddress.allCases.forEach(doTest)
	}
	
	
    func test_manifest_set_owner_keys_hashes() {
        func doTest(_ address: AddressOfAccountOrPersona, keyHashes: [PublicKeyHash]) {
            let manifest = SUT.setOwnerKeys(addressOfAccountOrPersona: address, ownerKeyHashes: keyHashes)
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
            let manifest = SUT.createFungibleTokenWithMetadata(
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
            
            let manifest = SUT.createFungibleToken(addressOfOwner: accountAddress)
			
            XCTAssertEqual(manifest.description.ranges(of: "Stella").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "STAR").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "The brightest component in the Radix ecosystem.").count, 1)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.allCases.forEach(doTest)
	}

    func test_create_multiple_fungible_tokens() {
        func doTest(_ accountAddress: AccountAddress) {
            let manifest = SUT.createMultipleFungibleTokens(addressOfOwner: accountAddress)
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
            let manifest = SUT.createNonFungibleToken(addressOfOwner: accountAddress)
			XCTAssertEqual(manifest.description.ranges(of: "An amazingly innovative and rare NFT collection").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "nf-number").count, 20)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.allCases.forEach(doTest)
	}

	func test_create_multiple_nft_collection() {
		func doTest(_ accountAddress: AccountAddress) {
            let manifest = SUT.createMultipleNonFungibleTokens(addressOfOwner: accountAddress)
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
            let manifest = SUT.stakesClaim(accountAddress: accountAddress, stakeClaims: StakeClaim.allCases)
			XCTAssertEqual(manifest.description.ranges(of: StakeClaim.sample.validatorAddress.mapTo(networkID: accountAddress.networkID).address).count, 1)
			XCTAssertEqual(manifest.description.ranges(of: accountAddress.xrd.address).count, 2)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		
		AccountAddress.allCases.forEach(doTest)
	}
    
    func test_update_third_party_deposits() {
        func doTest(_ accountAddress: AccountAddress) {
            let manifest = SUT.thirdPartyDepositUpdate(
                accountAddress: accountAddress,
                from: .sample,
                to: .sampleOther
            )
            XCTAssert(manifest.description.contains(accountAddress.address))
            XCTAssertEqual(manifest.description.ranges(of: ";").count, 3) // 3 instructions
        }
        AccountAddress.allCases.forEach(doTest)
    }
    
    func test_modify_manifest_add_lock_fee() throws {
		func doTest(_ addressOfFeePayer: AccountAddress) throws {
			var manifest = try rtm("create_pool")
			func hasLockFee() -> Bool {
				manifest.description.contains("lock_fee")
			}
			XCTAssertFalse(hasLockFee())
			let fee: Decimal192 = 531
			manifest = manifest.modify(lockFee: fee, addressOfFeePayer: addressOfFeePayer)
			XCTAssertTrue(hasLockFee())
			XCTAssert(manifest.description.contains(addressOfFeePayer.address))
		}

		try [
			AccountAddress.sampleStokenet,
			AccountAddress.sampleStokenetOther,
		].forEach(doTest)
    }
	
	func test_modify_manifest_add_guarantee() throws {
		var manifest = try rtm("transfer_1to2_multiple_nf_and_f_tokens")
		
		let guarantee = TransactionGuarantee(
			amount: 642,
			instructionIndex: 12,
			resourceAddress: .sampleStokenetXRD,
			resourceDivisibility: nil
		)

		XCTAssertFalse(manifest.description.contains(guarantee.amount.description))
		manifest = manifest.modify(addGuarantees: [guarantee])
		XCTAssertTrue(manifest.description.contains(guarantee.amount.description))
		
	}
	
	func test_assets_transfers() throws {
		let transfers = PerAssetTransfers.sample
		let manifest = TransactionManifest.assetsTransfers(transfers: transfers)
		XCTAssert(manifest.description.contains(transfers.fromAccount.address))
		transfers.fungibleResources.forEach {
			XCTAssert(manifest.description.contains($0.resource.resourceAddress.address))
			$0.transfers.forEach {
				XCTAssert(manifest.description.contains($0.recipient.description))
			}
		}
	}
	
	func rtm(_ rtm_file: String) throws -> TransactionManifest {
		let testsDirectory: String = URL(fileURLWithPath: "\(#file)").pathComponents.dropLast(6).joined(separator: "/")
		
		let fileURL = try XCTUnwrap(URL(fileURLWithPath: "\(testsDirectory)/src/wrapped_radix_engine_toolkit/low_level/transaction_manifest/execution_summary/\(rtm_file).rtm"))
		
		let data = try Data(contentsOf: fileURL)
		let instructionsString = try XCTUnwrap(String(data: data, encoding: .utf8))
		
		return try TransactionManifest(
			instructionsString: instructionsString,
			networkID: .stokenet,
			blobs: []
		)
	}
}

