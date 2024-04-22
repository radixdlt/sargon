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
	public init(arrayLiteral elements: Element...) {
		self.init(elements)
	}
}

extension CanBeEmptyIdentifiedCollection {
	public mutating func remove(_ id: Element.ID) {
		self = removing(id)
	}
	
	public mutating func remove(element: Element) {
		self = removing(element: element)
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
