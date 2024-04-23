//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension AuthorizedDapps: CanBeEmptyIdentifiedCollection {
	public typealias Element = AuthorizedDapp
	public mutating func append(_ newElement: Self.Element) {
		self = appending(newElement)
	}
}
