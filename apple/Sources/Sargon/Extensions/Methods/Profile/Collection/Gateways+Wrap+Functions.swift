import Foundation
import SargonUniFFI

extension Gateways {
	
	public init(_ elements: [Element]) {
		self = newGateways(gateways: elements)
	}
	
	public init(element: Element) {
		self = newGatewaysWithGateway(gateway: element)
	}
	
	public func allElements() -> [Element] {
		gatewaysGetElements(gateways: self)
	}
	
	public func appending(_ element: Element) -> Self {
		newGatewaysByAppending(gateway: element, to: self)
	}
	
	public func updatingOrInserting(element gateway: Element, at index: Int) -> Self {
		newGatewaysByUpdatingOrInsertingAtIndex(gateway: gateway, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ element: Element) -> Self {
		newGatewaysByUpdatingOrAppending(gateway: element, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newGatewaysRemovedById(idOfGateway: id, from: self)
	}
	
	public func removing(element: Element) -> Self {
		newGatewaysRemovedElement(gateway: element, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		gatewaysGetGatewayById(gateways: self, id: id)
	}
	
	public var count: Int {
		Int(gatewaysElementCount(gateways: self))
	}
}
