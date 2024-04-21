//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-04-19.
//

import Foundation
import SargonUniFFI

extension FiatCurrency: SargonModel {}
extension FiatCurrency: SargonStringCodable {}

extension FiatCurrency {
	public var rawValue: String {
		jsonStringLiteral()
	}
}

extension FiatCurrency: CustomStringConvertible {
	public var description: String {
		jsonStringLiteral()
	}
}
