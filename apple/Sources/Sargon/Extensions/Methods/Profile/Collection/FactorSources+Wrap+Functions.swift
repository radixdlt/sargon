//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension FactorSources {
	public init(_ elements: [FactorSource]) throws {
		self = try newFactorSources(factorSources: elements)
	}
	
	public init(element: FactorSource) {
		self = newFactorSourcesWithFactorSource(factorSource: element)
	}
	
	public var elements: [FactorSource] {
		factorSourcesGetElements(factorSources: self)
	}
	
	public func appending(_ factorSource: FactorSource) -> Self {
		newFactorSourcesByAppending(factorSource: factorSource, to: self)
	}
	
	public func updatingOrInserting(element factorSource: Element, at index: Int) -> Self {
		newFactorSourcesByUpdatingOrInsertingAtIndex(factorSource: factorSource, to: self, index: UInt64(index))
	}
	
	public func updatingOrAppending(_ factorSource: FactorSource) -> Self {
		newFactorSourcesByUpdatingOrAppending(factorSource: factorSource, to: self)
	}
	
	/// FactorSources is NonEmpty, so this throws if the resulting collection would be empty when removing
	/// the element by ID would result in an empty copy.
	public func removing(_ id: FactorSource.ID) throws -> Self {
		try newFactorSourcesRemovedById(idOfFactorSource: id, from: self)
	}
	
	/// FactorSources is NonEmpty, so this throws if the resulting collection would be empty when removing
	/// the element would result in an empty copy.
	public func removing(element factorSource: FactorSource) throws -> Self {
		try newFactorSourcesRemovedElement(factorSource: factorSource, from: self)
	}
	
	
	public func get(id: FactorSource.ID) -> FactorSource? {
		factorSourcesGetFactorSourceById(factorSources: self, id: id)
	}
	
	public var count: Int {
		Int(factorSourcesElementCount(factorSources: self))
	}
}
