import SargonUniFFI

public protocol DerivationPathProtocol: HDPathProtocol {
    var asDerivationPath: DerivationPath { get }
}

extension DerivationPath: DerivationPathProtocol {
    public init(string: String) throws {
        if let cap26 = try? CAP26Path(string: string) {
            self = .cap26(value: cap26)
        } else {
            self = try .bip44Like(value: .init(string: string))
        }
    }
    public var asDerivationPath: DerivationPath { self }
}

public protocol CAP26PathProtocol: DerivationPathProtocol {
    var asGeneral: CAP26Path { get }
}

extension CAP26PathProtocol {
    public var asDerivationPath: DerivationPath {
        .cap26(value: asGeneral)
    }
    public func toString() -> String {
        asGeneral.toString()
    }
}

extension CAP26PathProtocol {
    public var description: String {
      toString()
    }
}


