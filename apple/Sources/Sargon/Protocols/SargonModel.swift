public protocol SargonModel: Sendable, Hashable, CustomStringConvertible {
    #if DEBUG
    static var sample: Self { get }
    static var sampleOther: Self { get }
    #endif
}

#if DEBUG
extension SargonModel where Self: CaseIterable, AllCases == [Self] {
	public static var allCases: AllCases { [Self.sample, Self.sampleOther] }
}
#endif
