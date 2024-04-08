import SargonUniFFI

extension Gateway: SargonModel {}

extension Gateway {
	public static let mainnet = gatewayMainnet()
	public static let stokenet = gatewayStokenet()
}
