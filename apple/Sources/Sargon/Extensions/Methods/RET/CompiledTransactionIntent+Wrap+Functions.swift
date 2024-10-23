import Foundation
import SargonUniFFI

extension CompiledTransactionIntent {
	public var data: Data {
		compiledTransactionIntentBytes(compiledIntent: self)
	}
}
