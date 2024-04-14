//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension ProfileNetworks {
	public init(_ elements: [ProfileNetwork]) throws {
		self = try newProfileNetworks(profileNetworks: elements)
	}
	
	public var elements: [ProfileNetwork] {
		getProfileNetworks(profileNetworks: self)
	}
}