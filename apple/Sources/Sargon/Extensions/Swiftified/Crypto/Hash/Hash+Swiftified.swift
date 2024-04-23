import Foundation
import SargonUniFFI

extension Hash: SargonModel {}

extension Hash {
    
    public var hex: String {
        data.hex
    }
    
    public func hash() -> Self {
        data.hash()
    }

    public var data: Data {
        bytes.data
    }
}

extension Hash: CustomStringConvertible {
    public var description: String {
        hex
    }
}
