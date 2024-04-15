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
	
	public init(element: EntityFlag) {
		self = newEntityFlagsWithEntityFlag(entityFlag: element)
	}
	
	public var elements: [EntityFlag] {
		entityFlagsGetElements(entityFlags: self)
	}
	
	public func appending(_ entityFlag: EntityFlag) -> Self {
		newEntityFlagsByAppending(entityFlag: entityFlag, to: self)
	}
	
	public func removingElementByID(_ id: EntityFlag) -> Self {
		newEntityFlagsRemovedById(idOfEntityFlag: id, from: self)
	}
	
	public func removing(element flag: EntityFlag) -> Self {
		newEntityFlagsRemovedElement(entityFlag: flag, from: self)
	}
	
	public func get(id: EntityFlag.ID) -> EntityFlag? {
		entityFlagsGetEntityFlagById(entityFlags: self, id: id)
	}
	
	public var count: Int {
		Int(entityFlagsElementCount(entityFlags: self))
	}
}
