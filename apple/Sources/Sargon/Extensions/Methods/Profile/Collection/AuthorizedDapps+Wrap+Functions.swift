//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AuthorizedDapps {
	public init(_ elements: [AuthorizedDapp]) {
		self = newAuthorizedDapps(authorizedDapps: elements)
	}
	
	public init(element: AuthorizedDapp) {
		self = newAuthorizedDappsWithAuthorizedDapp(authorizedDapp: element)
	}
	
	public var elements: [AuthorizedDapp] {
		authorizedDappsGetElements(authorizedDapps: self)
	}
	
	public func appending(_ authorizedDapp: AuthorizedDapp) -> Self {
		newAuthorizedDappsByAppending(authorizedDapp: authorizedDapp, to: self)
	}
	
	public func removingElementByID(_ id: AuthorizedDapp.ID) -> Self {
		newAuthorizedDappsRemovedById(idOfAuthorizedDapp: id, from: self)
	}
	
	public func removing(element dapp: AuthorizedDapp) -> Self {
		newAuthorizedDappsRemovedElement(authorizedDapp: dapp, from: self)
	}
	
	public func get(id: AuthorizedDapp.ID) -> AuthorizedDapp? {
		authorizedDappsGetAuthorizedDappById(authorizedDapps: self, id: id)
	}
	
	public var count: Int {
		Int(authorizedDappsElementCount(authorizedDapps: self))
	}
}
