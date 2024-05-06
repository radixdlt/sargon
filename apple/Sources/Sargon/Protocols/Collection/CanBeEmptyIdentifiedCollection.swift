//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol CanBeEmptyIdentifiedCollection:
	BaseIdentifiedCollection,
	ExpressibleByArrayLiteral
where
	ArrayLiteralElement == Self.Element
{
	init(_ elements: [Element])
	
	func removing(_ id: Element.ID) -> Self
	func removing(element: Element) -> Self
}

extension CanBeEmptyIdentifiedCollection {
	public init() {
		self.init([])
	}

	
	// This is already implemented on `BaseIdentifiedCollection`,
	// but due to a Swift compiler bug in Xcode 15.3 we MUST implement
	// it here too 🤷‍♂️. 
	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
	
}

extension CanBeEmptyIdentifiedCollection {
	public init(arrayLiteral elements: Element...) {
		self.init(elements)
	}
}

extension CanBeEmptyIdentifiedCollection {
	@discardableResult
	public mutating func remove(_ id: Element.ID) -> Element? {
		let removed = get(id: id)
		self = removing(id)
		return removed
	}
	
	@discardableResult
	public mutating func remove(element: Element) -> Element? {
		let removed = get(id: element.id)
		self = removing(element: element)
		return removed
	}
}

extension CanBeEmptyIdentifiedCollection {
	public subscript(
		id id: Element.ID
	) -> Element? {
		get { self.get(id: id) }
		set {
			if let newValue {
				precondition(newValue.id == id)
				self.updateOrAppend(newValue)
				assert(contains(id: id))
			} else {
				self.remove(id)
				assert(!contains(id: id))
			}
		}
	}
}
