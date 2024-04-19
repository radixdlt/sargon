//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceID = FactorSourceId
extension FactorSourceID: SargonModel {}

extension FactorSourceID: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
