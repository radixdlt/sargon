import Foundation
import SargonUniFFI

// MARK: - PassphraseFactorSource + SargonModel
extension PassphraseFactorSource: SargonModel {}

// MARK: - PassphraseFactorSource + Identifiable
extension PassphraseFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

// MARK: - PassphraseFactorSource + FactorSourceProtocol
extension PassphraseFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .passphrase

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .passphrase(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.passphrase(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.passphrase
	}

	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
