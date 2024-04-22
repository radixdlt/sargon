import SargonUniFFI

public typealias CAP26Path = Cap26Path

extension CAP26Path: SargonModel, CAP26PathProtocol {
    public var path: HdPath {
        asDerivationPath.path
    }
    
    public var asGeneral: CAP26Path {
        self
    }
}

