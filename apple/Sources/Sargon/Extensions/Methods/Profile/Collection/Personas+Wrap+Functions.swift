//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Personas {
	public init(_ elements: [Persona]) {
		self = newPersonas(personas: elements)
	}
	
	public var elements: [Persona] {
		personasGetElements(personas: self)
	}
	
	public func appending(_ persona: Persona) -> Self {
		newPersonasByAppending(persona: persona, to: self)
	}
	
	public func removingPersona(id: Persona.ID) -> Self {
		newPersonasRemovedById(idOfPersona: id, from: self)
	}
	
	public func removing(persona: Persona) -> Self {
		newPersonasRemovedElement(persona: persona, from: self)
	}
	
	public func personaByID(_ id: Persona.ID) -> Persona? {
		personasGetPersonaById(personas: self, id: id)
	}
	
	public var count: Int {
		Int(personasElementCount(personas: self))
	}
}
