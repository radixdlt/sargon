//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension FactorSourceIDFromAddress {
	public func toString() -> String {
		factorSourceIdFromAddressToString(factorSourceId: self)
	}
	
	public init(jsonData: some DataProtocol) throws {
		self = try newFactorSourceIDFromAddressFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		factorSourceIDFromAddressToJsonBytes(factorSourceIDFromAddress: self)
	}
}
