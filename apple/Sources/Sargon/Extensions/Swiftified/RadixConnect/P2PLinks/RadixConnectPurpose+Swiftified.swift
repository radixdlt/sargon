import Foundation
import SargonUniFFI

extension RadixConnectPurpose: SargonModel {}
extension RadixConnectPurpose: CustomStringConvertible {
    public var description: String {
        toString()
    }
}

extension RadixConnectPurpose {
    
    public var rawValue: String {
        toString()
    }

    public init?(rawValue: String) {
        try? self.init(string: rawValue)
    }
}

