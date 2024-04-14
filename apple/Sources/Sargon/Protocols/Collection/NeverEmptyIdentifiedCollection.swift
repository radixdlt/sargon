//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol NeverEmptyIdentifiedCollection: BaseIdentifiedCollection {
	init(_ elements: [Element]) throws
	init(element: Element)
}
