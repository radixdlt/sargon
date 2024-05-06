//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Personas {
	public init(_ elements: [Element]) {
		self = newPersonas(personas: elements)
	}
	
	public init(element: Element) {
		self = newPersonasWithPersona(persona: element)
	}
	
	public func allElements() -> [Element] {
		personasGetElements(personas: self)
	}
	
	public func appending(_ persona: Element) -> Self {
		newPersonasByAppending(persona: persona, to: self)
	}
	
	public func updatingOrInserting(element persona: Element, at index: Int) -> Self {
		newPersonasByUpdatingOrInsertingAtIndex(persona: persona, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ persona: Element) -> Self {
		newPersonasByUpdatingOrAppending(persona: persona, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newPersonasRemovedById(idOfPersona: id, from: self)
	}
	
	public func removing(element persona: Element) -> Self {
		newPersonasRemovedElement(persona: persona, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		personasGetPersonaById(personas: self, id: id)
	}
	
	public var count: Int {
		Int(personasElementCount(personas: self))
	}
}
