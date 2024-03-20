extension AccountAddress: EntityAddressProtocol {
	public init(
		validatingAddress bech32String: String
	) throws {
		self = try newAccountAddress(bech32: bech32String)
	}

	public init(
		publicKey: PublicKey,
		networkID: NetworkID
	) {
		self = newAccountAddressFrom(
			publicKey: publicKey,
			networkId: networkID
		)
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
	/// `acco...please`
	///
	/// For the account address:
	///
	/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
	///
	public var shortFormat: String {
		accountAddressToShort(address: self)
	}
}
