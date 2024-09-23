@testable import Sargon

import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class TransactionManifestTests: Test<TransactionManifest> {

    func test_manifest_instructions_string() {
		let manifest = TransactionManifest.sample
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
	}

    func test_manifest_network_id() {
		let manifest = TransactionManifest.sample
        XCTAssertNoDifference(manifest.networkID, .mainnet)
	}

    func test_manifest_blobs() {
		let manifest = TransactionManifest.sample
        XCTAssertNoDifference(manifest.blobs, [])
	}
    
    func test_involved_resource_addresses() {
        XCTAssertNoDifference(SUT.sample.involvedResourceAddresses, [ResourceAddress.sampleMainnetXRD])
    }
    
    func test_involved_pool_addresses() {
        XCTAssertNoDifference(SUT.sample.involvedPoolAddresses, [])
    }
    
    func test_manifest_summary() {
        XCTAssertNoDifference(SUT.sample.summary.addressesOfAccountsWithdrawnFrom, [AccountAddress.sampleMainnet])
    }
    
    func test_execution_summary() throws {
        let name = "third_party_deposits_update"
		let engineToolkitReceiptStr = """
		  {
			  "kind": "CommitSuccess",
			  "state_updates_summary": {
				"new_entities": [],
				"metadata_updates": {},
				"non_fungible_data_updates": {},
				"newly_minted_non_fungibles": []
			  },
			  "worktop_changes": {},
			  "fee_summary": {
				"execution_fees_in_xrd": "0.07638415",
				"finalization_fees_in_xrd": "0.0105008",
				"storage_fees_in_xrd": "0.03871917658",
				"royalty_fees_in_xrd": "0"
			  },
			  "locked_fees": {
				"contingent": "0",
				"non_contingent": "0"
			  }
			}
		"""
		let receipt = engineToolkitReceiptStr.data(using: .utf8)!
        let manifest = try rtm(name)
        
        let summary = try manifest.executionSummary(engineToolkitReceipt: receipt)
        
        XCTAssertNoDifference(summary.addressesOfAccountsRequiringAuth, ["account_tdx_2_129uv9r46an4hwng8wc97qwpraspvnrc7v2farne4lr6ff7yaevaz2a"])
    }
	
	func test_from_instructions_string_with_max_sbor_depth_is_ok() throws {
		let instructionsString = """
CALL_METHOD
	Address("component_tdx_2_1crllllllllllllllllllllllllllllllllllllllllllllllx8navn")
	"dummy"
	Tuple(
		Tuple(
			Tuple(
				Tuple(
					Tuple(
						Tuple(
							Tuple(
								Tuple(
									Tuple(
										Tuple(
											Tuple(
												Tuple(
													Tuple(
														Tuple(
															Tuple(
																Tuple(
																	Tuple(
																		Tuple(
																			Tuple(
																				Tuple()
																			)
																		)
																	)
																)
															)
														)
													)
												)
											)
										)
									)
								)
							)
						)
					)
				)
			)
		)
	)
;
"""
		let sut = try SUT(instructionsString: instructionsString, networkID: .stokenet, blobs: []) // should be OK
		let intent = TransactionIntent(header: .sample, manifest: sut, message: .sample)
		XCTAssertEqual(intent.hash().description, "txid_rdx1uwcfczupvvrrtxwxx6p5jugaxvu3j83tj5nz9pnrr44jyxccg2cqhuvzhy")
	}
	
	func test_from_instructions_string_with_exceeded_sbor_depth_throws() {
		let instructionsString = """
CALL_METHOD
	Address("component_tdx_2_1crllllllllllllllllllllllllllllllllllllllllllllllx8navn")
	"dummy"
	Tuple(
		Tuple(
			Tuple(
				Tuple(
					Tuple(
						Tuple(
							Tuple(
								Tuple(
									Tuple(
										Tuple(
											Tuple(
												Tuple(
													Tuple(
														Tuple(
															Tuple(
																Tuple(
																	Tuple(
																		Tuple(
																			Tuple(
																				Tuple(
																					Tuple()
																				)
																			)
																		)
																	)
																)
															)
														)
													)
												)
											)
										)
									)
								)
							)
						)
					)
				)
			)
		)
	)
;
"""
		XCTAssertThrowsError(try SUT(instructionsString: instructionsString, networkID: .stokenet, blobs: []))
	}
}
