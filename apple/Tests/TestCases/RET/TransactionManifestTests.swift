@testable import Sargon

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
        let name = "transfer_1to2_multiple_nf_and_f_tokens"
        let receipt = try encodedReceipt(name)
        let manifest = try rtm(name)
        
        let summary = try manifest.executionSummary(encodedReceipt: receipt)
        
        XCTAssertNoDifference(summary.addressesOfAccountsRequiringAuth, ["account_tdx_2_1288efhmjt8kzce77par4ex997x2zgnlv5qqv9ltpxqg7ur0xpqm6gk"])
    }
	
	func test_from_instructions_string_with_max_sbor_depth_is_ok() throws {
		let instructionsString = """
CALL_METHOD
	Address("component_tdx_2_1czk50msdtutnznlupse7z5dpd3hhdeueey8l9t2uj54azqj6etqw3u")
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
		let intent = TransactionIntent(header: .sample, manifest: sut, message: .sample))
		XCTAssertEqual(intent.hash().description, "txid_rdx1umjkqnmsjh77p3xqqjwmpvtxyn055v23zuyjt70w6y85g3ej3w6sq9uh52")
	}
	
	func test_from_instructions_string_with_exceeded_sbor_depth_throws() {
		let instructionsString = """
CALL_METHOD
	Address("component_tdx_2_1czk50msdtutnznlupse7z5dpd3hhdeueey8l9t2uj54azqj6etqw3u")
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
