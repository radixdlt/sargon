import Foundation
import SargonUniFFI

extension RadixConnectPurpose: SargonModel {}
extension RadixConnectPurpose: SargonObjectCodable {}
extension RadixConnectPurpose: CustomStringConvertible {
    public var description: String {
        toString()
    }
}

extension RadixConnectPurpose {
    
    public var rawValue: String {
        toString()
    }

    public init(rawValue: String) {
        self.init(string: rawValue)
    }
}

