//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol CanBeEmptyIdentifiedCollection: BaseIdentifiedCollection & ExpressibleByArrayLiteral {
	init(_ elements: [Element])
}

extension CanBeEmptyIdentifiedCollection {
	public init(arrayLiteral elements: Element...) {
		self.init(elements)
	}
}
