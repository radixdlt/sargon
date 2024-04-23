//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-23.
//

import Foundation
import SargonUniFFI

extension RequestedQuantity {
	public var isValid: Bool {
		requestedQuantityIsValid(requestedQuantity: self)
	}
	
	public func isFulfilled(by numberOfIds: Int) -> Bool {
		requestedQuantityIsFulfilledByIds(requestedQuantity: self, numberOfIds: UInt64(numberOfIds))
	}
	
	public init(jsonData: some DataProtocol) throws {
		self = try newRequestedQuantityFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		requestedQuantityToJsonBytes(requestedQuantity: self)
	}

}
