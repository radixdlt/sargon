import Foundation
import SargonUniFFI

extension TransactionIntent {
	public func hash() -> TransactionIntentHash {
		transactionIntentHash(intent: self)
	}

	public func compile() -> CompiledTransactionIntent {
		transactionIntentCompile(intent: self)
	}
}
