import Foundation
import SargonUniFFI

// MARK: - BaseFactorSourceProtocol
public protocol BaseFactorSourceProtocol: SargonModel {
	var factorSourceID: FactorSourceID { get }
	var factorSourceKind: FactorSourceKind { get }
	var asGeneral: FactorSource { get }
	var supportsOlympia: Bool { get }
	var supportsBabylon: Bool { get }
	var common: FactorSourceCommon { get set }
}

extension BaseFactorSourceProtocol {
	public var kind: FactorSourceKind {
		factorSourceKind
	}

	public var cryptoParameters: FactorSourceCryptoParameters {
		common.cryptoParameters
	}

	public var addedOn: Date {
		common.addedOn
	}

	public var lastUsedOn: Date {
		common.lastUsedOn
	}

	public mutating func flag(_ flag: FactorSourceFlag) {
		common.flags.append(flag)
	}

	public var isFlaggedForDeletion: Bool {
		common.flags.contains(.deletedByUser)
	}
}

// MARK: - FactorSourceProtocol
public protocol FactorSourceProtocol: BaseFactorSourceProtocol {
	static var kind: FactorSourceKind { get }
	static func extract(from: some BaseFactorSourceProtocol) -> Self?
}

extension FactorSourceProtocol {
	public var factorSourceKind: FactorSourceKind {
		Self.kind
	}
}
