import Foundation
import SargonUniFFI

extension TransactionIntent {
	public func hash() -> IntentHash {
		transactionIntentHash(intent: self)
	}
	
	public func compile() -> CompiledTransactionIntent {
		transactionIntentCompile(intent: self)
	}
}
