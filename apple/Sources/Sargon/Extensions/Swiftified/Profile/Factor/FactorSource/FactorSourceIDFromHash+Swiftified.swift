//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromHash = FactorSourceIdFromHash

extension FactorSourceIDFromHash: SargonModel {}
extension FactorSourceIDFromHash: SargonObjectCodable {}

extension FactorSourceIDFromHash: FactorSourceIDSpecificProtocol {
	public var asGeneral: FactorSourceID {
		.hash(value: self)
	}
	public static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self? {
		guard case let .hash(id) = someFactorSourceID.asGeneral else { return nil }
		return id
	}
}


