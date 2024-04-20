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
	func updatingOrAppending(_ item: Element) -> Self
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
	
	/// Adds the given element to the array unconditionally, either appending it to the array, or
	/// replacing an existing value if it's already present.
	///
	/// - Parameter item: The value to append or replace.
	/// - Returns: The original element that was replaced by this operation, or `nil` if the value was
	///   appended to the end of the collection.
	/// - Complexity: The operation is expected to perform amortized O(1) copy, hash, and compare
	///   operations on the `ID` type, if it implements high-quality hashing.
	@inlinable
	@discardableResult
	mutating func updateOrAppend(_ item: Element) -> Element? {
		let originalElement = get(id: item.id)

		let countBefore = count
		self = updatingOrAppending(item)
		let countAfter = count
		
		
		if countAfter != countBefore {
			// appended
			return nil
		} else {
			// the original element which was replaced
			return originalElement
		}
	}
}
