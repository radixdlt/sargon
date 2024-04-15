//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension ProfileNetworks {
	
	public init(_ elements: [ProfileNetwork]) {
		self = newProfileNetworks(profileNetworks: elements)
	}
	
	public init(element: ProfileNetwork) {
		self = newProfileNetworksWithProfileNetwork(profileNetwork: element)
	}
	
	public var elements: [ProfileNetwork] {
		profileNetworksGetElements(profileNetworks: self)
	}
	
	public func appending(_ network: ProfileNetwork) -> Self {
		newProfileNetworksByAppending(profileNetwork: network, to: self)
	}
	
	public func removingElementByID(_ id: ProfileNetwork.ID) -> Self {
		newProfileNetworksRemovedById(idOfProfileNetwork: id, from: self)
	}
	
	public func removing(element network: ProfileNetwork) -> Self {
		newProfileNetworksRemovedElement(profileNetwork: network, from: self)
	}
	
	public func get(id: ProfileNetwork.ID) -> ProfileNetwork? {
		profileNetworksGetProfileNetworkById(profileNetworks: self, id: id)
	}
	
	public var count: Int {
		Int(profileNetworksElementCount(profileNetworks: self))
	}
}
