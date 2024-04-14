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
		getFactorSources(factorSources: self)
	}
}
