import SargonUniFFI

// MARK: - SavedGateways + SargonModel
extension SavedGateways: SargonModel {}
extension SavedGateways {
	public static let preset: Self = .default
}
