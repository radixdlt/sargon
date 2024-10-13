import Foundation
import SargonUniFFI

extension DappToWalletInteractionUnvalidated {
    public func toJSONString() -> String {
        dappToWalletInteractionUnvalidatedToJsonString(dappToWalletInteractionUnvalidated: self)
	}

    public init(jsonString: String) throws {
        self = try newDappToWalletInteractionUnvalidatedFromJsonString(jsonString: jsonString)
	}
}
