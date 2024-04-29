import Foundation
import SargonUniFFI

// MARK: - FiatCurrency + SargonModel
extension FiatCurrency: SargonModel {}

// MARK: - FiatCurrency + SargonStringCodable
extension FiatCurrency: SargonStringCodable {}

extension FiatCurrency {
	public var rawValue: String {
		jsonStringLiteral()
	}
}

// MARK: - FiatCurrency + CustomStringConvertible
extension FiatCurrency: CustomStringConvertible {
	public var description: String {
		jsonStringLiteral()
	}
}
