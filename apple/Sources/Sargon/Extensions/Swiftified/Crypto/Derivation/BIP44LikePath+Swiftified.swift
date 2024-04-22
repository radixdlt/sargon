import Foundation
import SargonUniFFI

public typealias BIP44LikePath = Bip44LikePath

extension BIP44LikePath: SargonModel, DerivationPathProtocol {
    public var asDerivationPath: DerivationPath {
        .bip44Like(value: self)
    }
}

extension BIP44LikePath: CustomStringConvertible {
    public var description: String {
        toString()
    }
}
