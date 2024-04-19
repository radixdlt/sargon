//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol BaseIdentifiedCollection:
	SargonModel,
	RandomAccessCollection
where 
	Index == Array<Element>.Index,
	Element: Identifiable,
	Element: SargonModel
{
	var elements: [Element] { get }
	init(element: Element)
	func appending(_ element: Element) -> Self
	func get(id: Element.ID) -> Element?
}

extension BaseIdentifiedCollection {
	public var startIndex: Index {
		elements.startIndex
	}
	
	public var endIndex: Index {
		elements.endIndex
	}
	
	public func index(after index: Index) -> Index {
		elements.index(after: index)
	}
	
	public subscript(position: Index) -> Element {
		elements[position]
	}
	
	public mutating func append(_ element: Element) {
		self = appending(element)
	}
	
}
