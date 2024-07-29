//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension TrustedContactFactorSource: SargonModel {}
extension TrustedContactFactorSource: Identifiable {
	public typealias ID = FactorSourceIDFromAddress
}

extension TrustedContactFactorSource: FactorSourceProtocol {
	public static let kind: FactorSourceKind = .trustedContact
	
	public static func extract(from someFactorSource: some BaseFactorSourceProtocol) -> Self? {
		guard case let .trustedContact(factorSource) = someFactorSource.asGeneral else { return nil }
		return factorSource
	}
	
	public var asGeneral: FactorSource {
		.trustedContact(value: self)
	}
	
	public var factorSourceID: FactorSourceID {
		id.asGeneral
	}
	
	public var factorSourceKind: FactorSourceKind {
		.trustedContact
	}
	
	public var supportsOlympia: Bool { asGeneral.supportsOlympia }
	public var supportsBabylon: Bool { asGeneral.supportsBabylon }
}
