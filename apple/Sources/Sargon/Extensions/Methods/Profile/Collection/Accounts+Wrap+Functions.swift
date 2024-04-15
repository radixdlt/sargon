//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Accounts {
	public init(_ elements: [Account]) {
		self = newAccounts(accounts: elements)
	}
	
	public var elements: [Account] {
		accountsGetElements(accounts: self)
	}
	
	public func appending(_ account: Account) -> Self {
		newAccountsByAppending(account: account, to: self)
	}
	
	public func removingElementByID(_ id: Account.ID) -> Self {
		newAccountsRemovedById(idOfAccount: id, from: self)
	}
	
	public func removing(element account: Account) -> Self {
		newAccountsRemovedElement(account: account, from: self)
	}
	
	public func get(id: Account.ID) -> Account? {
		accountsGetAccountById(accounts: self, id: id)
	}
	
	internal var elementCount: Int {
		Int(accountsElementCount(accounts: self))
	}
	
	public var count: Int {
		elementCount
	}
}
