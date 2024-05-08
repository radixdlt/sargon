import Foundation
import SargonUniFFI

extension LinkConnectionQrData {
    public init(jsonData: some DataProtocol) throws {
        self = try newLinkConnectionQRDataFromJsonBytes(jsonBytes: Data(jsonData))
    }
    
    public func jsonData() -> Data {
        linkConnectionQRDataToJsonBytes(linkConnectionQRData: self)
    }
}
