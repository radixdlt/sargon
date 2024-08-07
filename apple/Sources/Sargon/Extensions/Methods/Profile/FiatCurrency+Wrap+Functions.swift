//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

// FiatCurrency -> SargonStringCodable
extension FiatCurrency {
	public init(jsonStringLiteral: String) throws {
		self = try newFiatCurrencyFromJsonString(jsonString: jsonStringLiteral)
	}
	
	public func jsonStringLiteral() -> String {
		fiatCurrencyToJsonString(fiatCurrency: self)
	}
}
