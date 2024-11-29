import Foundation
import SargonUniFFI

// MARK: - PrivateHierarchicalDeterministicFactorSource + SargonModel
extension PrivateHierarchicalDeterministicFactorSource: SargonModel {}

// MARK: - PrivateHierarchicalDeterministicFactorSource + BaseFactorSourceProtocol
extension PrivateHierarchicalDeterministicFactorSource: BaseFactorSourceProtocol {
	public var common: FactorSourceCommon {
		get { factorSource.common }
		set {
			factorSource.common = newValue
		}
	}

	public var factorSourceID: FactorSourceID {
		factorSource.factorSourceID
	}

	public var factorSourceKind: FactorSourceKind {
		factorSource.factorSourceKind
	}

	public var asGeneral: FactorSource {
		factorSource.asGeneral
	}

	public var supportsOlympia: Bool {
		factorSource.supportsOlympia
	}

	public var supportsBabylon: Bool {
		factorSource.supportsBabylon
	}
}
