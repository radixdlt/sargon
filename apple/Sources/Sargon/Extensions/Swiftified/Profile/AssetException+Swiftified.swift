//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension AssetException: SargonModel {}

extension AssetException: Identifiable {
	public typealias ID = ResourceAddress
	public var id: ID {
		address
	}
}
