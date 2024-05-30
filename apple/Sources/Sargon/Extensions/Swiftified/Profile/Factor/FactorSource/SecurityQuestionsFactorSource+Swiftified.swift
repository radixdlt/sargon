//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension SecurityQuestionsNotProductionReadyFactorSource: SargonModel {}
extension SecurityQuestionsNotProductionReadyFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

extension SecurityQuestionsNotProductionReadyFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .securityQuestions
	
	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .securityQuestions(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}
	
	public var asGeneral: FactorSource {
		.securityQuestions(value: self)
	}
	
	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
