import Foundation
import SargonUniFFI

public typealias BIP44LikePath = Bip44LikePath

// MARK: SargonModel, DerivationPathProtocol
extension BIP44LikePath: SargonModel, DerivationPathProtocol {
	public var asGeneral: DerivationPath {
		.bip44Like(value: self)
	}

	public var asDerivationPath: DerivationPath {
		.bip44Like(value: self)
	}
}

// MARK: CustomStringConvertible
extension BIP44LikePath: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
