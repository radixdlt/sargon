import Foundation
import SargonUniFFI

// MARK: - PassphraseFactorSource + SargonModel
extension PasswordFactorSource: SargonModel {}

// MARK: - PassphraseFactorSource + Identifiable
extension PasswordFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

// MARK: - PassphraseFactorSource + FactorSourceProtocol
extension PasswordFactorSource: FactorSourceProtocol {
    public static let kind: FactorSourceKind = .password

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .password(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.password(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.password
	}

	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
