import Foundation
import SargonUniFFI

// MARK: - CompiledNotarizedIntent + SargonModel
extension CompiledNotarizedIntent: SargonModel {}

// MARK: - CompiledNotarizedIntent + CustomStringConvertible
extension CompiledNotarizedIntent: CustomStringConvertible {
	public var description: String {
		data.hex
	}
}
