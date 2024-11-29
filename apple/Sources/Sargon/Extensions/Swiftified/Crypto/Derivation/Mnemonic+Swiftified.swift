import Foundation
import SargonUniFFI

// MARK: - Mnemonic + SargonModel
extension Mnemonic: SargonModel {}

// MARK: - Mnemonic + CustomStringConvertible
extension Mnemonic: CustomStringConvertible {
	public var description: String {
		phrase
	}
}
