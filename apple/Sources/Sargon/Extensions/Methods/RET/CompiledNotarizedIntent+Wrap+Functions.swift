import Foundation
import SargonUniFFI

extension CompiledNotarizedIntent {
	public var data: Data {
		compiledNotarizedIntentGetBytes(compiledNotarizedIntent: self)
	}
}
