import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class TransactionManifestTests: Test<TransactionManifest> {
//	func test_manifest_string() {
//		let manifest = SUT.sample
//		XCTAssert(manifest.manifestString.contains("CALL_METHOD"))
//	}

	func test_manifest_instructions_string() {
		let manifest = TransactionManifest.sample
		XCTAssert(manifest.instructionsString.contains("CALL_METHOD"))
	}

	func test_manifest_network_id() {
		let manifest = TransactionManifest.sample
		XCTAssertNoDifference(manifest.networkId, .mainnet)
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
		XCTAssertEqual(intent.hash().description, "txid_rdx1mecs336t36x7rcuyhwa0jtsef08jq5r573nfa63t8f4aq9faxrls2n3u3c")
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
