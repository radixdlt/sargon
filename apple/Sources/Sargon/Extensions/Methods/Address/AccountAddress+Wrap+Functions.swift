import SargonUniFFI

extension AccountAddress {
	public init(
		validatingAddress bech32String: String
	) throws {
		self = try newAccountAddress(bech32: bech32String)
	}

	public init(
		publicKey: some PublicKeyProtocol,
		networkID: NetworkID
	) {
		self = newAccountAddressFrom(
			publicKey: publicKey.asGeneral,
			networkId: networkID
		)
	}

	public func formatted(_ format: AddressFormat = .default) -> String {
		accountAddressFormatted(address: self, format: format)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		accountAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accountAddressNetworkId(address: self)
	}

	/// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
	/// for all accounts created by the Babylon Radix Wallets.
	/// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
	/// imported from the Olympia Wallet.
	public var isLegacy: Bool {
		accountAddressIsLegacy(address: self)
	}

	/// Formats the AccountAddress to its abbreviated form which is what the user
	/// is most used to, since it is what we most commonly display in the Radix
	/// ecosystem.
	///
	/// The abbreviated form returns:
	///
	/// `acco...nvjdwr`
	///
	/// For the account address:
	///
	/// `account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr`
	///
	public var shortFormat: String {
		formatted(AddressFormat.default)
	}
}

#if DEBUG
extension AccountAddress {
	public static func random(networkID: NetworkID) -> Self {
		newAccountAddressRandom(networkId: networkID)
	}

	public func mapTo(networkID: NetworkID) -> Self {
		accountAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
