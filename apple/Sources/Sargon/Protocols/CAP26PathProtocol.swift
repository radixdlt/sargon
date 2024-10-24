import SargonUniFFI

public protocol DerivationPathProtocol: HDPathProtocol {
    var asGeneral: DerivationPath { get }
}



extension DerivationPathProtocol {
    public func toString() -> String {
        asGeneral.toString()
    }
    public var description: String {
      toString()
    }
}


