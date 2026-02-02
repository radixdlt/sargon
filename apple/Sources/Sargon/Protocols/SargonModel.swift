// MARK: - BaseSargonModel
public protocol BaseSargonModel: Sendable, Hashable {}

#if DEBUG
public protocol SargonModel: BaseSargonModel {
	static var sample: Self { get }
	static var sampleOther: Self { get }
}
#else
public protocol SargonModel: BaseSargonModel {}
#endif // if DEBUG

#if DEBUG
extension SargonModel {
	public static var sampleValues: [Self] {
		[sample, sampleOther]
	}
}
#endif // DEBUG
