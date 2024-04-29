import Foundation

// MARK: - NeverEmptyIdentifiedCollection
public protocol NeverEmptyIdentifiedCollection: BaseIdentifiedCollection {
	init(_ elements: [Element]) throws

	func removing(_ id: Element.ID) throws -> Self
	func removing(element: Element) throws -> Self
}

extension NeverEmptyIdentifiedCollection {
	public mutating func remove(_ id: Element.ID) throws {
		self = try removing(id)
	}

	public mutating func remove(element: Element) throws {
		self = try removing(element: element)
	}
}
