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
	
	public init(element: Persona) {
		self = newPersonasWithPersona(persona: element)
	}
	
	public var elements: [Persona] {
		personasGetElements(personas: self)
	}
	
	public func appending(_ persona: Persona) -> Self {
		newPersonasByAppending(persona: persona, to: self)
	}
	
	public func updatingOrAppending(_ persona: Persona) -> Self {
		newPersonasByUpdatingOrAppending(persona: persona, to: self)
	}
	
	public func removingElementByID(_ id: Persona.ID) -> Self {
		newPersonasRemovedById(idOfPersona: id, from: self)
	}
	
	public func removing(element persona: Persona) -> Self {
		newPersonasRemovedElement(persona: persona, from: self)
	}
	
	public func get(id: Persona.ID) -> Persona? {
		personasGetPersonaById(personas: self, id: id)
	}
	
	public var count: Int {
		Int(personasElementCount(personas: self))
	}
}
