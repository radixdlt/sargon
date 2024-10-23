import Foundation
import SargonUniFFI

extension Subintent {
    public func hash() -> SubintentHash {
        subintentHash(intent: self)
    }

    public func compile() -> Data {
        subintentCompile(intent: self)
    }
}
