import Foundation
import SargonUniFFI

extension CompiledSubintent {
	public var data: Data {
		compiledSubintentBytes(compiledIntent: self)
	}
}
