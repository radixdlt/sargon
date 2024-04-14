//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation

public protocol BaseIdentifiedCollection: RandomAccessCollection where Index == Array<Element>.Index, Element: Identifiable {
	var elements: [Element] { get }
}

extension BaseIdentifiedCollection {
	public var startIndex: Index {
		elements.startIndex
	}
	
	public var endIndex: Index {
		elements.endIndex
	}
	
	public func index(after index: Index) -> Index {
		elements.index(after: index)
	}
	
	public subscript(position: Index) -> Element {
		elements[position]
	}
	
}
