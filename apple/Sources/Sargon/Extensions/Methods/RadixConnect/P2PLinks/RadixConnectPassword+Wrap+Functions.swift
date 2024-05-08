import Foundation
import SargonUniFFI

extension RadixConnectPassword {
    public init(jsonData: some DataProtocol) throws {
        self = try newRadixConnectPasswordFromJsonBytes(jsonBytes: Data(jsonData))
    }
    
    public func jsonData() -> Data {
        radixConnectPasswordToJsonBytes(radixConnectPassword: self)
    }
}
