import SargonUniFFI

extension CAP26Path {
    public init(string: String) throws {
        self = try newCap26PathFromString(string: string)
    }
    
    public func toString() -> String {
        cap26PathToString(path: self)
    }
}
