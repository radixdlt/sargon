//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Accounts: CanBeEmptyIdentifiedCollection {
	public typealias Element = Account
}

// MARK: RangeReplaceableCollection
extension Accounts {

	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
	

}
