import Foundation
import SargonUniFFI

extension DappToWalletInteractionUnvalidated {
    public init(jsonData: some DataProtocol) throws {
        self = try newDappToWalletInteractionUnvalidatedFromJsonBytes(jsonBytes: Data(jsonData))
    }
    
    public func jsonData() -> Data {
        dappToWalletInteractionUnvalidatedToJsonBytes(dappToWalletInteractionUnvalidated: self)
    }
}
