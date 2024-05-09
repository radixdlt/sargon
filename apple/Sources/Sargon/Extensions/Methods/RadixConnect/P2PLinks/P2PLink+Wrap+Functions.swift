import Foundation
import SargonUniFFI

extension P2pLink {
    public init(jsonData: some DataProtocol) throws {
        self = try newP2PLinkFromJsonBytes(jsonBytes: Data(jsonData))
    }
    
    public func jsonData() -> Data {
        p2PLinkToJsonBytes(p2PLink: self)
    }
}