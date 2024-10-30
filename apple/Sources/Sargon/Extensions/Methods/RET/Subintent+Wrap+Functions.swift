import Foundation
import SargonUniFFI

extension Subintent {
    public func hash() -> SubintentHash {
      subintentHash(subintent: self)
    }

    public func compile() -> CompiledSubintent {
        subintentCompile(subintent: self)
    }
}
