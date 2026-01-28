import CustomDump
import Foundation
import RegexBuilder
@testable import Sargon
import SargonUniFFI
import XCTest

// MARK: - ManifestBuildingTests
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
		AccountAddress.sampleValues.forEach(doTest)
	}

	func test_manifest_set_owner_keys_hashes() {
		func doTest(_ address: AddressOfAccountOrPersona, keyHashes: [PublicKeyHash]) {
			let manifest = SUT.setOwnerKeys(addressOfAccountOrPersona: address, ownerKeyHashes: keyHashes)
			XCTAssert(manifest.description.contains(address.description))
			XCTAssert(manifest.description.contains("SET_METADATA"))
			XCTAssert(manifest.description.contains("owner_keys"))
		}

		for sampleValue in AddressOfAccountOrPersona.sampleValues {
			doTest(sampleValue, keyHashes: [.sample])
			doTest(sampleValue, keyHashes: [.sample, .sampleOther])
			doTest(sampleValue, keyHashes: [.sampleOther])
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
			func oneIn(metadata keyPath: KeyPath<TokenDefinitionMetadata, some CustomStringConvertible>, line: UInt = #line) {
				let property = metadata[keyPath: keyPath]
				oneOf(property.description, line: line)
			}
			oneIn(metadata: \.name)
			oneIn(metadata: \.description)
			oneIn(metadata: \.symbol)
			oneOf(initialSupply.formattedPlain(locale: .test))
		}
		// We are not testing with AccountAddress.sampleValues since sampleMainnet & sampleMainnetOther are used
		// to build the prefilled the dummy metadata extra fields (so they will appear more than once in the manifest).
		AccountAddress.sampleValues.forEach(doTest)
	}

	func test_create_single_fungible_token() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = SUT.createFungibleToken(addressOfOwner: accountAddress)

			XCTAssertEqual(manifest.description.ranges(of: "Stella").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "STAR").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "The brightest component in the Radix ecosystem.").count, 1)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.sampleValues.forEach(doTest)
	}

	func test_create_multiple_fungible_tokens() {
		func doTest(_ accountAddress: AccountAddress) {
			let n: UInt8 = 5
			let manifest = SUT.createMultipleFungibleTokens(
				addressOfOwner: accountAddress,
				count: n
			)
			XCTAssertEqual(manifest.description.ranges(of: "symbol").count, Int(n))
			XCTAssert(manifest.description.contains(accountAddress.address))
		}

		[
			AccountAddress.sampleStokenet,
			AccountAddress.sampleStokenetOther,
		].forEach(doTest)
	}

	func test_create_single_nft_collection() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = SUT.createNonFungibleToken(addressOfOwner: accountAddress)
			XCTAssertEqual(manifest.description.ranges(of: "An amazingly innovative and rare NFT collection").count, 1)
			XCTAssertEqual(manifest.description.ranges(of: "nf-number").count, 20)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}
		AccountAddress.sampleValues.forEach(doTest)
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
		AccountAddress.sampleValues.forEach(doTest)
	}

	func test_stakes_claim() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = SUT.stakesClaim(accountAddress: accountAddress, stakeClaims: StakeClaim.sampleValues)
			XCTAssertEqual(manifest.description.ranges(of: StakeClaim.sample.validatorAddress.mapTo(networkID: accountAddress.networkID).address).count, 1)
			XCTAssertEqual(manifest.description.ranges(of: accountAddress.xrdOnSameNetwork.address).count, 2)
			XCTAssert(manifest.description.contains(accountAddress.address))
		}

		AccountAddress.sampleValues.forEach(doTest)
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
		AccountAddress.sampleValues.forEach(doTest)
	}

	func test_assets_transfers() {
		let transfers = PerAssetTransfers.sample
		let manifest = TransactionManifest.assetsTransfers(transfers: transfers)
		XCTAssert(manifest.description.contains(transfers.fromAccount.address))
		for fungibleResource in transfers.fungibleResources {
			XCTAssert(manifest.description.contains(fungibleResource.resource.resourceAddress.address))
			for transfer in fungibleResource.transfers {
				XCTAssert(manifest.description.contains(transfer.recipient.description))
			}
		}
	}

	func test_account_locker_claim() {
		func doTest(_ accountAddress: AccountAddress) {
			let manifest = SUT.accountLockerClaim(
				lockerAddress: LockerAddress.sample,
				claimant: accountAddress,
				claimableResources: [AccountLockerClaimableResource.fungible(resourceAddress: ResourceAddress.sample, amount: Decimal192.sample)]
			)
			XCTAssert(manifest.description.contains(accountAddress.address))
			XCTAssertEqual(manifest.description.ranges(of: ";").count, 3) // 3 instructions
		}
		AccountAddress.sampleValues.forEach(doTest)
	}
}

extension TestCase {
	func engineToolkitReceipt(_ name: String) throws -> String {
		let utf8 = try openTransactionFile(name, extension: "dat")
		return try XCTUnwrap(String(data: utf8, encoding: .utf8))
	}

	func rtm(
		_ rtmFile: String,
		in crate: String = "transaction/models"
	) throws -> TransactionManifest {
		let data = try openTransactionFile(rtmFile, extension: "rtm", in: crate)
		let instructionsString = try XCTUnwrap(String(data: data, encoding: .utf8))

		return try TransactionManifest(
			instructionsString: instructionsString,
			networkID: .stokenet,
			blobs: []
		)
	}

	private func openTransactionFile(
		_ fileName: String,
		extension fileExtension: String,
		in crate: String = "transaction/models"
	) throws -> Data {
		try openFile(subPath: "transaction", fileName, extension: fileExtension)
	}
}
