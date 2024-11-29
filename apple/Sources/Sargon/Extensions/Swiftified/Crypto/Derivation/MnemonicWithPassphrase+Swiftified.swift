import Foundation
import SargonUniFFI

// MARK: - MnemonicWithPassphrase + SargonModel
extension MnemonicWithPassphrase: SargonModel {}

// MARK: - MnemonicWithPassphrase + SargonObjectCodable
extension MnemonicWithPassphrase: SargonObjectCodable {}

extension MnemonicWithPassphrase {
	public func derivePublicKey(
		path: some DerivationPathProtocol
	) -> HierarchicalDeterministicPublicKey {
		derivePublicKeys(paths: [path]).first!
	}
}
