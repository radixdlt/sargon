//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

extension FactorSourceIDFromHash {
	public func toString() -> String {
		factorSourceIdFromHashToString(factorSourceId: self)
	}
	
	public init(jsonData: some DataProtocol) throws {
		self = try newFactorSourceIDFromHashFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		factorSourceIDFromHashToJsonBytes(factorSourceIDFromHash: self)
	}
}
