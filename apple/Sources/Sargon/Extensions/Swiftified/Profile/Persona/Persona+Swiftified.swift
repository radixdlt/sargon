//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension Persona: EntityProtocol {
	public typealias EntityAddress = IdentityAddress
	
	public var asGeneral: AccountOrPersona {
		.persona(self)
	}
}

extension Persona: EntitySpecificProtocol {
	public static let kind: EntityKind = .persona
	public static func extract(from someEntity: some EntityProtocol) -> Self? {
		guard case let .persona(persona) = someEntity.asGeneral else { return nil }
		return persona
	}
}
