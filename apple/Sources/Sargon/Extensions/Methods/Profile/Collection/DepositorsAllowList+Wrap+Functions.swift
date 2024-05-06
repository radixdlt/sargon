//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI


extension DepositorsAllowList {
	
	public init(_ elements: [Element]) {
		self = newDepositorsAllowList(depositorsAllowList: elements)
	}
	
	public init(element: Element) {
		self = newDepositorsAllowListWithResourceOrNonFungible(resourceOrNonFungible: element)
	}
	
	public func allElements() -> [Element] {
		depositorsAllowListGetElements(depositorsAllowList: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newDepositorsAllowListByAppending(resourceOrNonFungible: element, to: self)
	}
	
	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newDepositorsAllowListByUpdatingOrInsertingAtIndex(
			resourceOrNonFungible: element,
			to: self,
			index: UInt64(
				index
			)
		)
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newDepositorsAllowListByUpdatingOrAppending(resourceOrNonFungible: element, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newDepositorsAllowListRemovedById(idOfResourceOrNonFungible: id, from: self)
	}
	
	public func removing(element: Element) -> Self {
		newDepositorsAllowListRemovedElement(resourceOrNonFungible: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		depositorsAllowListGetResourceOrNonFungibleById(depositorsAllowList: self, id: id)
	}
	
	public var count: Int {
		Int(depositorsAllowListElementCount(depositorsAllowList: self))
	}
}
