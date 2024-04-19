import Foundation
import SargonUniFFI

extension Hash: SargonModel {}

extension Hash {
    
    public var hex: String {
        data.hex
    }
}

extension Hash: CustomStringConvertible {
    public var description: String {
        hex
    }
}
