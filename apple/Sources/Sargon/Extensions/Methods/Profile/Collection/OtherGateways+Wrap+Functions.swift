import Foundation
import SargonUniFFI

extension OtherGateways {
	
	public init(_ elements: [Element]) {
		self = newOtherGateways(otherGateways: elements)
	}
	
	public init(element: Element) {
		self = newOtherGatewaysWithGateway(gateway: element)
	}
	
	public var elements: [Element] {
		otherGatewaysGetElements(otherGateways: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newOtherGatewaysByAppending(gateway: element, to: self)
	}
	
	public func updatingOrInserting(element gateway: Element, at index: Int) -> Self {
		newOtherGatewaysByUpdatingOrInsertingAtIndex(gateway: gateway, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newOtherGatewaysByUpdatingOrAppending(gateway: element, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newOtherGatewaysRemovedById(idOfGateway: id, from: self)
	}
	
	public func removing(element: Element) -> Self {
		newOtherGatewaysRemovedElement(gateway: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		otherGatewaysGetGatewayById(otherGateways: self, id: id)
	}
	
	public var count: Int {
		Int(otherGatewaysElementCount(otherGateways: self))
	}
}
