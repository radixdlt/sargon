import Foundation
import SargonUniFFI

extension UnvalidatedTransactionManifest {
    public init(manifest: TransactionManifest) {
		self = newUnvalidatedTransactionManifestFromTransactionManifest(transactionManifest: manifest)
	}

    public func transactionManifest(onNetwork networkID: NetworkID) throws -> TransactionManifest {
        try newTransactionManifest(
            unvalidatedTransactionManifest: self, 
            networkId: networkID
        )
    }
}
