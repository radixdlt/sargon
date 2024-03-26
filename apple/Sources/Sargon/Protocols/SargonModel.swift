public protocol BaseSargonModel: Sendable, Hashable {}

#if DEBUG
public protocol SargonModel: BaseSargonModel, CaseIterable {
    static var sample: Self { get }
    static var sampleOther: Self { get }
}
#else
public protocol SargonModel: BaseSargonModel {}
#endif


#if DEBUG
extension SargonModel {
	public static var allCases: [Self] { [Self.sample, Self.sampleOther] }
}
#endif
