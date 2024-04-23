//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-22.
//

import Foundation
import SargonUniFFI

extension AssetsExceptionList: SargonModel {}
extension AssetsExceptionList: CanBeEmptyIdentifiedCollection {
	public typealias Element = AssetException
	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
}
