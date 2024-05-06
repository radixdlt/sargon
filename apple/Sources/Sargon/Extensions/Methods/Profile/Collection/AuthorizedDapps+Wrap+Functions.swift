//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AuthorizedDapps {
	public init(_ elements: [Element]) {
		self = newAuthorizedDapps(authorizedDapps: elements)
	}
	
	public init(element: Element) {
		self = newAuthorizedDappsWithAuthorizedDapp(authorizedDapp: element)
	}
	
	public func allElements() -> [Element] {
		authorizedDappsGetElements(authorizedDapps: self)
	}
	
	public func appending(_ authorizedDapp: Element) -> Self {
		newAuthorizedDappsByAppending(authorizedDapp: authorizedDapp, to: self)
	}
	
	public func updatingOrInserting(element authorizedDapp: Element, at index: Int) -> Self {
		newAuthorizedDappsByUpdatingOrInsertingAtIndex(authorizedDapp: authorizedDapp, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ authorizedDapp: Element) -> Self {
		newAuthorizedDappsByUpdatingOrAppending(authorizedDapp: authorizedDapp, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newAuthorizedDappsRemovedById(idOfAuthorizedDapp: id, from: self)
	}
	
	public func removing(element dapp: Element) -> Self {
		newAuthorizedDappsRemovedElement(authorizedDapp: dapp, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		authorizedDappsGetAuthorizedDappById(authorizedDapps: self, id: id)
	}
	
	public var count: Int {
		Int(authorizedDappsElementCount(authorizedDapps: self))
	}
}
