import Foundation
import SargonUniFFI

// MARK: - ArculusCardModel + CustomStringConvertible
extension ArculusCardModel: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - ArculusCardFactorSource + SargonModel
extension ArculusCardFactorSource: SargonModel {}

// MARK: - ArculusCardFactorSource + Identifiable
extension ArculusCardFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

// MARK: - ArculusCardFactorSource + FactorSourceProtocol
extension ArculusCardFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .arculusCard

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .arculusCard(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.arculusCard(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.arculusCard
	}

	public var supportsOlympia: Bool {
		asGeneral.supportsOlympia
	}

	public var supportsBabylon: Bool {
		asGeneral.supportsBabylon
	}
}
