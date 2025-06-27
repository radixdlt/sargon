// import Foundation
// import SargonUniFFI
//
//// MARK: - SecurityQuestionsNotProductionReadyFactorSource + SargonModel
// extension SecurityQuestionsNotProductionReadyFactorSource: SargonModel {}
//
//// MARK: - SecurityQuestionsNotProductionReadyFactorSource + Identifiable
// extension SecurityQuestionsNotProductionReadyFactorSource: Identifiable {
//	public typealias ID = FactorSourceIDFromHash
// }
//
//// MARK: - SecurityQuestionsNotProductionReadyFactorSource + FactorSourceProtocol
// extension SecurityQuestionsNotProductionReadyFactorSource: FactorSourceProtocol {
//	public static let kind: FactorSourceKind = .securityQuestions
//
//	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
//		guard case let .securityQuestions(factorSource) = someFactorSource.asGeneral else { return nil }
//		return factorSource
//	}
//
//	public var asGeneral: FactorSource {
//		.securityQuestions(value: self)
//	}
//
//	public var factorSourceID: FactorSourceID {
//		id.asGeneral
//	}
//
//	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
//	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
// }
