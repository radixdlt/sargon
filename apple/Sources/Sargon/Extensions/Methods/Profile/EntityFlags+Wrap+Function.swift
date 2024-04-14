//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension EntityFlags {
	public init(_ elements: [EntityFlag]) {
		self = newEntityFlags(entityFlags: elements)
	}
	
	public var elements: [EntityFlag] {
		getEntityFlags(entityFlags: self)
	}
}
