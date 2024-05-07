import Foundation
import SargonUniFFI

extension DetailedAuthorizedPersonas {
	public init(_ elements: [Element]) {
		self = newDetailedAuthorizedPersonas(detailedAuthorizedPersonas: elements)
	}
	
	public init(element: Element) {
		self = newDetailedAuthorizedPersonasWithAuthorizedPersonaDetailed(authorizedPersonaDetailed: element)
	}
	
	public var elements: [Element] {
		detailedAuthorizedPersonasGetElements(detailedAuthorizedPersonas: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newDetailedAuthorizedPersonasByAppending(authorizedPersonaDetailed: element, to: self)
	}
	
	public func updatingOrInserting(element: Element, at index: Int) -> Self {
		newDetailedAuthorizedPersonasByUpdatingOrInsertingAtIndex(authorizedPersonaDetailed: element, to: self, index: UInt64(index))
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newDetailedAuthorizedPersonasRemovedById(idOfAuthorizedPersonaDetailed: id, from: self)
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newDetailedAuthorizedPersonasByUpdatingOrAppending(authorizedPersonaDetailed: element, to: self)
	}
	
	public func removing(element: Element) -> Self {
		newDetailedAuthorizedPersonasRemovedElement(authorizedPersonaDetailed: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		detailedAuthorizedPersonasGetAuthorizedPersonaDetailedById(detailedAuthorizedPersonas: self, id: id)
	}
	
	public var count: Int {
		Int(detailedAuthorizedPersonasElementCount(detailedAuthorizedPersonas: self))
	}
}
