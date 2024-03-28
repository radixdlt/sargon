import SargonUniFFI

extension IntentHash {
    public init(string: String) throws {
        self = try newIntentHashFromString(string: string)
    }
}
