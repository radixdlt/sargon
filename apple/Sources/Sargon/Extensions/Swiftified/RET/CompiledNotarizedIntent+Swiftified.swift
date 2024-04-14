import Foundation
import SargonUniFFI

extension CompiledNotarizedIntent: SargonModel {}

extension CompiledNotarizedIntent: CustomStringConvertible {
	public var description: String {
		data.hex
	}
}
