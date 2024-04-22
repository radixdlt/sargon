//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI


extension ResourceOrNonFungible: SargonModel {}

extension ResourceOrNonFungible: Identifiable {
	public typealias ID = Self
	public var id: ID {
		self
	}
}

extension ResourceOrNonFungible {
	public var resourceAddress: ResourceAddress {

		switch self {
		case let .nonFungible(value): value.resourceAddress
		case let .resource(value): value
		}
	}
}
