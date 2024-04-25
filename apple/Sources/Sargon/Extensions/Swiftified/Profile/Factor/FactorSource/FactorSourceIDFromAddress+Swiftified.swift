//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromAddress = FactorSourceIdFromAddress

extension FactorSourceIDFromAddress: SargonModel {}
extension FactorSourceIDFromAddress: SargonObjectCodable {}

extension FactorSourceIDFromAddress: FactorSourceIDSpecificProtocol {
	public var asGeneral: FactorSourceID {
		.address(value: self)
	}
	public static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self? {
		guard case let .address(id) = someFactorSourceID.asGeneral else { return nil }
		return id
	}
}
