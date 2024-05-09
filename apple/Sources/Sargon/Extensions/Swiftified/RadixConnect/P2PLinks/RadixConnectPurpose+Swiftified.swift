import Foundation
import SargonUniFFI

extension RadixConnectPurpose: SargonModel {}
extension RadixConnectPurpose: SargonObjectCodable {}

extension RadixConnectPurpose {

    public init(rawValue: String) {
        self.init(string: rawValue)
    }
}

