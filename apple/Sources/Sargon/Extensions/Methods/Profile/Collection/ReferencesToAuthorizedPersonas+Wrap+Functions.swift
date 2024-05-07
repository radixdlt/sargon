//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension ReferencesToAuthorizedPersonas {
	public init(_ elements: [Element]) {
		self = newReferencesToAuthorizedPersonas(referencesToAuthorizedPersonas: elements)
	}
	
	public init(element: Element) {
		self = newReferencesToAuthorizedPersonasWithAuthorizedPersonaSimple(authorizedPersonaSimple: element)
	}
	
	public var elements: [Element] {
		referencesToAuthorizedPersonasGetElements(referencesToAuthorizedPersonas: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newReferencesToAuthorizedPersonasByAppending(authorizedPersonaSimple: element, to: self)
	}
	
	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newReferencesToAuthorizedPersonasByUpdatingOrInsertingAtIndex(authorizedPersonaSimple: element, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newReferencesToAuthorizedPersonasByUpdatingOrAppending(authorizedPersonaSimple: element, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newReferencesToAuthorizedPersonasRemovedById(idOfAuthorizedPersonaSimple: id, from: self)
	}
	
	public func removing(element: Element) -> Self {
		newReferencesToAuthorizedPersonasRemovedElement(authorizedPersonaSimple: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		referencesToAuthorizedPersonasGetAuthorizedPersonaSimpleById(referencesToAuthorizedPersonas: self, id: id)
	}
	
	public var count: Int {
		Int(referencesToAuthorizedPersonasElementCount(referencesToAuthorizedPersonas: self))
	}
}
