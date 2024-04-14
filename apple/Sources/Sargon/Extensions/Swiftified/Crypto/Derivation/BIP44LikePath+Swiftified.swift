import Foundation
import SargonUniFFI

public typealias BIP44LikePath = Bip44LikePath

extension BIP44LikePath: SargonModel, HDPathProtocol {}

extension BIP44LikePath: CustomStringConvertible {
    public var description: String {
        toString()
    }
}
