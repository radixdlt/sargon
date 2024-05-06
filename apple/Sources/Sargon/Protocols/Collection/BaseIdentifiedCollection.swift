//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol BaseIdentifiedCollection:
	SargonModel,
	Collection
where
	Index == Array<Element>.Index,
	Element: Identifiable,
	Element: SargonModel
{
	/// This is an EXPENSIVE operation,
	/// prefer using `self.ids()` and then just read out a subset of
	/// the elements with `get:id`, to only read a few instead of all.
	func allElements() -> [Element]
	
	init(element: Element)
	var count: Int { get }
	func appending(_ element: Element) -> Self
	func get(id: Element.ID) -> Element?
	func get(at position: Int) -> Element
	func updatingOrAppending(_ element: Element) -> Self
	func updatingOrInserting(element: Element, at index: Int) -> Self
}

extension BaseIdentifiedCollection {
	
	public func get(at position: Int) -> Element {
		fatalError("impl me")
	}
	
	public func contains(id: Element.ID) -> Bool {
		get(id: id) != nil
	}
	
	public var ids: [Element.ID] {
		fatalError("impl me")
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
			get(at: position)
		}
		set {
			updateOrInsert(element: newValue, at: position)
		}
	}
}

extension BaseIdentifiedCollection {
	public var startIndex: Index {
		0
	}
	
	public var endIndex: Index {
		count - 1
	}
	
	public func index(after i: Index) -> Index {
		i + 1
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
