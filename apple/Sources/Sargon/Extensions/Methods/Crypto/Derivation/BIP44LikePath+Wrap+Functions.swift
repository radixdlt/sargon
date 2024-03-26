extension BIP44LikePath {
    public init(string: String) throws {
        self = try newBip44LikePathFromString(string: string)
    }
    
    public func toString() -> String {
        bip44LikePathToString(path: self)
    }
}
