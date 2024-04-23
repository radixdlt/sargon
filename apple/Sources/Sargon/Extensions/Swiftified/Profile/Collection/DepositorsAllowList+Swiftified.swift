//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension DepositorsAllowList: SargonModel {}
extension DepositorsAllowList: CanBeEmptyIdentifiedCollection {
	public typealias Element = ResourceOrNonFungible
	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
}
