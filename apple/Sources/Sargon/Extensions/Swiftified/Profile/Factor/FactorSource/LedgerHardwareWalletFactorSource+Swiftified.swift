import Foundation
import SargonUniFFI

// MARK: - LedgerHardwareWalletFactorSource + SargonModel
extension LedgerHardwareWalletFactorSource: SargonModel {}

// MARK: - LedgerHardwareWalletFactorSource + Identifiable
extension LedgerHardwareWalletFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

// MARK: - LedgerHardwareWalletFactorSource + FactorSourceProtocol
extension LedgerHardwareWalletFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .ledgerHqHardwareWallet

	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .ledger(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}

	public var asGeneral: FactorSource {
		.ledger(value: self)
	}

	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}

	public var factorSourceKind: FactorSourceKind {
		.ledgerHqHardwareWallet
	}

	public var supportsOlympia: Bool {
		asGeneral.supportsOlympia
	}

	public var supportsBabylon: Bool {
		asGeneral.supportsBabylon
	}
}
