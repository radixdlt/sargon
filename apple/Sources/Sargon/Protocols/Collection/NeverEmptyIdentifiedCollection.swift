//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol NeverEmptyIdentifiedCollection: BaseIdentifiedCollection {
	init(_ elements: [Element]) throws
	init(element: Element)
	
	func removingElementByID(_ id: Element.ID) throws -> Self
	func removing(element: Element) throws -> Self
}

extension NeverEmptyIdentifiedCollection {
	
	public mutating func removeElementByID(_ id: Element.ID) throws {
		self = try removingElementByID(id)
	}
	
	public mutating func remove(element: Element) throws {
		self = try removing(element: element)
	}
}
