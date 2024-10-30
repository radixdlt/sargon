import Foundation
import SargonUniFFI

extension CompiledSubintent {
	public var data: Data {
		compiled_subintent_bytes(compiledIntent: self)
	}
}