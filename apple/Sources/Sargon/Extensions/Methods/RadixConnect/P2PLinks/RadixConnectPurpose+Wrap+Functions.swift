import Foundation
import SargonUniFFI

extension RadixConnectPurpose {
    
    public init(string: String) {
        self = newRadixConnectPurposeFromString(string: string)
    }
    
    public func toString() -> String {
        radixConnectPurposeToString(kind: self)
    }
}