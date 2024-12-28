import Foundation
import SargonUniFFI

// MARK: - AccountPath + SargonModel, DerivationPathProtocol
extension AccountPath: SargonModel, DerivationPathProtocol {}

extension AccountPath {
	public init(string: String) throws {
		switch try DerivationPath(string: string) {
		case let .account(value):
			self = value
		case .identity, .bip44Like:
			throw SargonError.WrongEntityKind(
				expected: Cap26EntityKind.account.description,
				found: Cap26EntityKind.identity.description
			)
		}
	}

	public var asGeneral: DerivationPath {
		.account(value: self)
	}
}
