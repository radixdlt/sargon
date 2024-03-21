#if DEBUG
public protocol SargonModel: Sendable, Hashable, CaseIterable {
    static var sample: Self { get }
    static var sampleOther: Self { get }
}
#else
public protocol SargonModel: Sendable, Hashable {}
#endif


#if DEBUG
extension SargonModel {
	public static var allCases: [Self] { [Self.sample, Self.sampleOther] }
}
#endif
