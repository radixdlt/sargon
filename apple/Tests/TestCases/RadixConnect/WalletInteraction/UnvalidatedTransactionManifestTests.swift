import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class UnvalidatedTransactionManifestTests: Test<UnvalidatedTransactionManifest> {
	func testTransactionManifestOnNetwork() throws {
		let instance = SUT(manifest: .sample)
		let result = try instance.transactionManifest(onNetwork: .sample)
		XCTAssertEqual(result, TransactionManifest.sample)
	}
}
