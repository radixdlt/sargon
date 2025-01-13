import Foundation
import SargonUniFFI

extension CompiledTransactionIntent {
	public var data: Data {
		compiledTransactionIntentBytes(compiledIntent: self)
	}

	public func decompile() -> TransactionIntent {
		compiledTransactionIntentDecompile(compiledIntent: self)
	}
}
