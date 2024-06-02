//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension ArculusCardModel: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

extension ArculusCardFactorSource: SargonModel {}
extension ArculusCardFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromHash
}

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
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
