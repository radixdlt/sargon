import SargonUniFFI

// MARK: - Gateways + SargonModel
extension Gateways: SargonModel {}
extension Gateways {
	public static let preset: Self = .default
}
