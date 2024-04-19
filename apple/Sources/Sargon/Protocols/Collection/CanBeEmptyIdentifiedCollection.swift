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
	
	func removingElementByID(_ id: Element.ID) -> Self
	func removing(element: Element) -> Self
}

extension CanBeEmptyIdentifiedCollection {
	public init(arrayLiteral elements: Element...) {
		self.init(elements)
	}
}

extension CanBeEmptyIdentifiedCollection {
	public mutating func removeElementByID(_ id: Element.ID) {
		self = removingElementByID(id)
	}
	
	public mutating func remove(element: Element) {
		self = removing(element: element)
	}
}
