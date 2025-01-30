// import Foundation
// import SargonUniFFI
//
//// MARK: - TrustedContactFactorSource + SargonModel
// extension TrustedContactFactorSource: SargonModel {}
//
//// MARK: - TrustedContactFactorSource + Identifiable
// extension TrustedContactFactorSource: Identifiable {
//	public typealias ID = FactorSourceIDFromAddress
// }
//
//// MARK: - TrustedContactFactorSource + FactorSourceProtocol
// extension TrustedContactFactorSource: FactorSourceProtocol {
//	public static let kind: FactorSourceKind = .trustedContact
//
//	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
//		guard case let .trustedContact(factorSource) = someFactorSource.asGeneral else { return nil }
//		return factorSource
//	}
//
//	public var asGeneral: FactorSource {
//		.trustedContact(value: self)
//	}
//
//	public var factorSourceID: FactorSourceID {
//		id.asGeneral
//	}
//
//	public var factorSourceKind: FactorSourceKind {
//		.trustedContact
//	}
//
//	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
//	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
// }
