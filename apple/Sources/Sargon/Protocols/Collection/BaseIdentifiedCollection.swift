//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol BaseIdentifiedCollection:
	SargonModel,
	RandomAccessCollection,
	MutableCollection
where
	Index == Array<Element>.Index,
	Element: Identifiable,
	Element: SargonModel
{
	var elements: [Element] { get }
	init(element: Element)
	mutating func append(_ newElement: Self.Element)
	func appending(_ element: Element) -> Self
	func get(id: Element.ID) -> Element?
	func updatingOrAppending(_ element: Element) -> Self
	func updatingOrInserting(element: Element, at index: Int) -> Self
}

extension BaseIdentifiedCollection {
	public func contains(id: Element.ID) -> Bool {
		get(id: id) != nil
	}
}


// MARK: MutableCollection
extension BaseIdentifiedCollection {
	
	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
	
	@inlinable
	@inline(__always)
	public subscript(position: Int) -> Element {
		get {
			elements[position]
		}
		set {
			updateOrInsert(element: newValue, at: position)
		}
	}
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
	
	@discardableResult
	public mutating func updateOrInsert(element: Element, at index: Int) -> (originalMember: Element?, index: Int) {
		let originalMember = get(id: element.id)
		self = updatingOrInserting(element: element, at: index)
		let deFactoIndex = self.firstIndex(where: { $0.id == element.id })!
		return (originalMember, index: deFactoIndex)
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
	public mutating func updateOrAppend(_ item: Element) -> Element? {
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
