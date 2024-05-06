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
	
	public init(element: Account) {
		self = newAccountsWithAccount(account: element)
	}
	
	public func allElements() -> [Element] {
		accountsGetElements(accounts: self)
	}
	
	public func appending(_ account: Account) -> Self {
		newAccountsByAppending(account: account, to: self)
	}
	
	public func updatingOrInserting(element account: Element, at index: Int) -> Self {
		newAccountsByUpdatingOrInsertingAtIndex(account: account, to: self, index: UInt64(index))
	}
	
	public func removing(_ id: Account.ID) -> Self {
		newAccountsRemovedById(idOfAccount: id, from: self)
	}
	
	public func updatingOrAppending(_ account: Account) -> Self {
		newAccountsByUpdatingOrAppending(account: account, to: self)
	}
	
	public func removing(element account: Account) -> Self {
		newAccountsRemovedElement(account: account, from: self)
	}
	
	public func get(id: Account.ID) -> Account? {
		accountsGetAccountById(accounts: self, id: id)
	}
	
	public var count: Int {
		Int(accountsElementCount(accounts: self))
	}
}
