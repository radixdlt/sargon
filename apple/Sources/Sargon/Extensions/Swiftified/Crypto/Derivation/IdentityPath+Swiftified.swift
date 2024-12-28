import Foundation
import SargonUniFFI

// MARK: - IdentityPath + SargonModel, DerivationPathProtocol
extension IdentityPath: SargonModel, DerivationPathProtocol {}

extension IdentityPath {
	public init(string: String) throws {
		switch try DerivationPath(string: string) {
		case let .identity(value):
			self = value
		case .account, .bip44Like:
			throw SargonError.WrongEntityKind(expected: "identity", found: "account")
		}
	}

	public var asGeneral: DerivationPath {
		.identity(value: self)
	}
}
