import Foundation
import SargonUniFFI

extension Mnemonic: SargonModel {}

extension Mnemonic: CustomStringConvertible {
	public var description: String {
		phrase
	}
}
