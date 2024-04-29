import Foundation
import SargonUniFFI

// MARK: - DerivationPath + SargonModel
extension DerivationPath: SargonModel {}

// MARK: - DerivationPath + CustomStringConvertible
extension DerivationPath: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - DerivationPath + DerivationPathProtocol
extension DerivationPath: DerivationPathProtocol {
	public var asDerivationPath: DerivationPath { self }
}

public typealias HDPath = HdPath

extension DerivationPath {
	/// Returns the index, non hardened, so `3H` returns `3`.
	public var nonHardenedIndex: HDPathValue {
		let component = self.path.components.last! // safe to unwrap, we disallow empty paths.
		return component.nonHardenedValue
	}

	public var curve: SLIP10Curve {
		switch self {
		case .bip44Like: .secp256k1
		case .cap26: .curve25519
		}
	}

	public static func forEntity(
		kind: EntityKind,
		networkID: NetworkID,
		index: HDPathValue
	) -> Self {
		switch kind {
		case .account:
			AccountPath(
				networkID: networkID,
				keyKind: .transactionSigning,
				index: index
			).asDerivationPath
		case .persona:
			IdentityPath(
				networkID: networkID,
				keyKind: .transactionSigning,
				index: index
			).asDerivationPath
		}
	}
}

extension HdPathComponent {
	public var nonHardenedValue: HDPathValue {
		hdPathComponentGetNonHardenedValue(component: self)
	}
}
