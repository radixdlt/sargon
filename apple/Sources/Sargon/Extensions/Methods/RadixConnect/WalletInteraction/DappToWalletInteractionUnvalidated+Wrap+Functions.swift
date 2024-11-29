import Foundation
import SargonUniFFI

extension DappToWalletInteractionUnvalidated {
	public func toJSONString() -> String {
		dappToWalletInteractionUnvalidatedToJsonString(interactionUnvalidated: self)
	}

	public init(jsonString: String) throws {
		self = try newDappToWalletInteractionUnvalidatedFromJsonString(jsonStr: jsonString)
	}
}
