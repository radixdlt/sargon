import Foundation
import SargonUniFFI

extension DappToWalletInteractionUnvalidated {
    public func toJSONString(prettyPrinted: Bool = false) -> String {
		dappToWalletInteractionUnvalidatedToJsonString(interactionUnvalidated: self, prettyPrinted: prettyPrinted)
	}

    public init(jsonString: String) throws {
		self = try newDappToWalletInteractionUnvalidatedFromJsonString(jsonStr: jsonString)
	}
}
