import Foundation
import SargonUniFFI

/// FiatCurrency -> SargonStringCodable
extension FiatCurrency {
	public init(jsonStringLiteral: String) throws {
		self = try newFiatCurrencyFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		fiatCurrencyToJsonString(fiatCurrency: self)
	}
}
