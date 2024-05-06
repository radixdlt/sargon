//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension ProfileNetworks {
	
	public init(_ elements: [Element]) {
		self = newProfileNetworks(profileNetworks: elements)
	}
	
	public init(element: Element) {
		self = newProfileNetworksWithProfileNetwork(profileNetwork: element)
	}
	
	public func allElements() -> [Element] {
		profileNetworksGetElements(profileNetworks: self)
	}
	
	public func appending(_ network: Element) -> Self {
		newProfileNetworksByAppending(profileNetwork: network, to: self)
	}
	
	public func updatingOrInserting(element profileNetwork: Element, at index: Int) -> Self {
		newProfileNetworksByUpdatingOrInsertingAtIndex(profileNetwork: profileNetwork, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ network: Element) -> Self {
		newProfileNetworksByUpdatingOrAppending(profileNetwork: network, to: self)
	}
	
	public func removing(_ id: Element.ID) -> Self {
		newProfileNetworksRemovedById(idOfProfileNetwork: id, from: self)
	}
	
	public func removing(element network: Element) -> Self {
		newProfileNetworksRemovedElement(profileNetwork: network, from: self)
	}
	
	public func get(id: Element.ID) -> Element? {
		profileNetworksGetProfileNetworkById(profileNetworks: self, id: id)
	}
	
	public var count: Int {
		Int(profileNetworksElementCount(profileNetworks: self))
	}
}
