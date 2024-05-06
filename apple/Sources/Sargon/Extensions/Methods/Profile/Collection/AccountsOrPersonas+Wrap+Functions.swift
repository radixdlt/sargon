//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AccountsOrPersonas {
	public init(_ elements: [Element]) {
		self = newAccountsOrPersonas(accountsOrPersonas: elements)
	}
	
	public init(element: Element) {
		self = newAccountsOrPersonasWithAccountOrPersona(accountOrPersona: element)
	}
	
	public func allElements() -> [Element] {
		accountsOrPersonasGetElements(accountsOrPersonas: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newAccountsOrPersonasByAppending(accountOrPersona: element, to: self)
	}
	
	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newAccountsOrPersonasByUpdatingOrInsertingAtIndex(accountOrPersona: element, to: self, index: UInt64(index))
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newAccountsOrPersonasRemovedById(idOfAccountOrPersona: id, from: self)
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newAccountsOrPersonasByUpdatingOrAppending(accountOrPersona: element, to: self)
	}
	
	public func removing(element: Element) -> Self {
		newAccountsOrPersonasRemovedElement(accountOrPersona: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		accountsOrPersonasGetAccountOrPersonaById(accountsOrPersonas: self, id: id)
	}
	
	public var count: Int {
		Int(accountsOrPersonasElementCount(accountsOrPersonas: self))
	}
}
