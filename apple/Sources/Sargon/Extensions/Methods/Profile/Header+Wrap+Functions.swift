//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension Header {
	public init(jsonData: some DataProtocol) throws {
		self = try newHeaderFromJsonBytes(jsonBytes: Data(jsonData))
	}
	
	public func jsonData() -> Data {
		headerToJsonBytes(header: self)
	}
}
