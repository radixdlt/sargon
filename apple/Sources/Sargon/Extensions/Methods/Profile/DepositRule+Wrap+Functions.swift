import Foundation
import SargonUniFFI

/// DepositRule -> SargonStringCodable
extension DepositRule {
	public init(jsonStringLiteral: String) throws {
		self = try newDepositRuleFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		depositRuleToJsonString(depositRule: self)
	}
}
