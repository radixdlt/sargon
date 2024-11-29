import Foundation
import SargonUniFFI

// MARK: - HierarchicalDeterministicFactorInstance + SargonModel
extension HierarchicalDeterministicFactorInstance: SargonModel {}

extension HierarchicalDeterministicFactorInstance {
	public var factorSourceID: FactorSourceIDFromHash {
		factorSourceId
	}

	public var derivationPath: DerivationPath {
		publicKey.derivationPath
	}

	public var factorInstance: FactorInstance {
		FactorInstance(
			factorSourceId: factorSourceID.asGeneral,
			badge: .virtual(
				value: .hierarchicalDeterministic(value: self.publicKey)
			)
		)
	}
}
