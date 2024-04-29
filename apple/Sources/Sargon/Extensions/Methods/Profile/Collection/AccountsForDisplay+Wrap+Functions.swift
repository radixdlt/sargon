import Foundation
import SargonUniFFI

extension AccountsForDisplay {
	public init(_ elements: [Element]) {
		self = newAccountsForDisplay(accountsForDisplay: elements)
	}

	public init(element: Element) {
		self = newAccountsForDisplayWithAccountForDisplay(accountForDisplay: element)
	}

	public var elements: [Element] {
		accountsForDisplayGetElements(accountsForDisplay: self)
	}

	public func appending(_ element: Element) -> Self {
		newAccountsForDisplayByAppending(accountForDisplay: element, to: self)
	}

	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newAccountsForDisplayByUpdatingOrInsertingAtIndex(accountForDisplay: element, to: self, index: UInt64(index))
	}

	public func removing(_ id: Element.ID) -> Self {
		newAccountsForDisplayRemovedById(idOfAccountForDisplay: id, from: self)
	}

	public func updatingOrAppending(_ element: Element) -> Self {
		newAccountsForDisplayByUpdatingOrAppending(accountForDisplay: element, to: self)
	}

	public func removing(element: Element) -> Self {
		newAccountsForDisplayRemovedElement(accountForDisplay: element, from: self)
	}

	public func get(id: Element.ID) -> Element? {
		accountsForDisplayGetAccountForDisplayById(accountsForDisplay: self, id: id)
	}

	public var count: Int {
		Int(accountsForDisplayElementCount(accountsForDisplay: self))
	}
}
