//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-25.
//

import Foundation
import SargonUniFFI

extension Entropy16Bytes {

	public init(bytes: some DataProtocol) throws {
		self = try newEntropy16BytesFromBytes(bytes: Data(bytes))
	}
	
	public var data: Data {
		entropy16BytesToBytes(bytes: self)
	}
}
