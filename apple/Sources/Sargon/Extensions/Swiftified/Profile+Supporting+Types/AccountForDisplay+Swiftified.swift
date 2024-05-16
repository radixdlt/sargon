//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-28.
//

import Foundation
import SargonUniFFI

extension AccountForDisplay: SargonModel {}

extension AccountForDisplay: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		address
	}
	
}
