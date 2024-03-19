public protocol SargonModel: Sendable, Hashable, CustomStringConvertible {
    #if DEBUG
    static var sample: Self { get }
    static var sampleOther: Self { get }
    #endif
}
