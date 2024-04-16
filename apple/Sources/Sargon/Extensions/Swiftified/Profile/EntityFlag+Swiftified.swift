//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension EntityFlag: SargonModel & Identifiable {
	public typealias ID = Self
	public var id: ID {
		self
	}
}
