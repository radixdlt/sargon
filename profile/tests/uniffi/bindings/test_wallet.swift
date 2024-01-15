import Foundation
import radix_wallet_kit

func test() throws {
	print("🚀 Test Wallet in Swift start")
	defer { print("✅ Test Wallet in Swift completed ") }

	let keychain = EphemeralKeychain.shared
	assert(keychain.isEmpty)

	// MARK: ==================================
	print("🔮 GENERATING NEW WALLET")
	// or: `Wallet.generateNew()`
	let wallet = try Wallet.with(
		entropy: Data(repeating: 0xff, count: 32)
	)

	assert(
		keychain.contains(
			value:
				"zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote"
		))
	print("✨ SUCCESSFULLY GENERATED NEW WALLET ✅")
	// MARK: ==================================
	print("🔮 Creating first account on mainnet")
	let initialNameOfFirstAccount = "Alice"
	// Not created any account yet...
	assert(!keychain.contains(value: initialNameOfFirstAccount))
	assert(wallet.profile().networks.count == 0)
	var main0 = try wallet.createAndSaveNewAccount(
		networkId: .mainnet,
		name: DisplayName(
			validating: initialNameOfFirstAccount
		)
	)
	assert(main0.networkId == .mainnet)
	assert(wallet.profile().networks.count == 1)
	assert(wallet.profile().networks[0].accounts.count == 1)
	assert(
		wallet.profile().networks[0].accounts[0].displayName.value
			== initialNameOfFirstAccount
	)
	assert(keychain.contains(value: initialNameOfFirstAccount))
	print("✨ Successfully created first account ✅")
	// MARK: ==================================
	print("🔮 Renaming account")
	let updatedNameOfFirstAccount = "Satoshi"
	main0 = try wallet.changeNameOfAccount(
		address: main0.address,
		to: DisplayName(
			validating: updatedNameOfFirstAccount
		))
	assert(
		wallet.profile().networks[0].accounts[0].displayName.value
			== updatedNameOfFirstAccount
	)
	assert(
		keychain.contains(
			value: updatedNameOfFirstAccount
		))
	print("✨ Successfully renamed first account ✅")
	// MARK: ==================================
	print("🔮 Creating second mainnet account")
	let main1 = try wallet.createAndSaveNewAccount(
		networkId: .mainnet,
		name: DisplayName(
			validating: "Bob"
		)
	)
	assert(main0.address != main1.address)
	assert(
		main0.networkId == main1.networkId
	)
	assert(wallet.profile().networks.count == 1)
	assert(wallet.profile().networks[0].accounts == [main0, main1])
	print("✨ Successfully created second mainnet account ✅")

	// MARK: ==================================
	print("🔮 Creating first testnet account")
	let testnetAccountName = "Hello Radix Account!"
	let test0 = try wallet.createAndSaveNewAccount(
		networkId: .stokenet,
		name: DisplayName(
			validating: testnetAccountName
		)
	)
	assert(wallet.profile().networks.count == 2)
	assert(wallet.profile().networks[1].accounts == [test0])
	assert(
		wallet.profile().networks[1].accounts[0].displayName.value
			== testnetAccountName
	)
	assert(
		wallet.profile().networks[1].accounts[0].networkId
			== .stokenet
	)
	assert(keychain.contains(value: testnetAccountName))

	print("✨ Successfully created first testnet account ✅")

}

try! test()

// MARK: Helpers

extension Profile {
	fileprivate static let placeholder = newProfilePlaceholder()
}
extension DisplayName {
	init(validating value: String) throws {
		self = try newDisplayName(name: value)
	}
}
extension Data {
	public static func random(byteCount: Int) throws -> Self {
		var bytes = [UInt8](repeating: 0, count: byteCount)
		let status = SecRandomCopyBytes(kSecRandomDefault, byteCount, &bytes)
		if status == errSecSuccess {
			return Self(bytes)
		}
		struct UnableToGenerateBytes: Swift.Error {}
		throw UnableToGenerateBytes()
	}
}

extension Wallet {
	public static let defaultIphoneName: String = "iPhone"

	public static func generateNew(
		iPhoneName: String = Wallet.defaultIphoneName
	) throws -> Wallet {
		try Wallet.with(
			entropy: .random(byteCount: 32), iPhoneName: iPhoneName
		)
	}

	public static func with(
		entropy: Data,
		iPhoneName: String = Wallet.defaultIphoneName
	) throws -> Wallet {
		// Rust: `by_creating_new_profile_and_secrets_with_entropy`
		try Wallet.byCreatingNewProfileAndSecretsWithEntropy(
			entropy: entropy,
			walletClientModel: .iphone,
			walletClientName: iPhoneName,
			secureStorage: EphemeralKeychain.shared
		)
	}
}

extension SecureStorageKey {
	var identifier: String {
		secureStorageKeyIdentifier(key: self)
	}
}

public final class EphemeralKeychain {
	private var store: [String: Data]
	private init() {
		store = [:]
	}
}
extension EphemeralKeychain: SecureStorage {
	public func loadData(key: SecureStorageKey) throws -> Data? {
		store[key.identifier]
	}
	public func saveData(key: SecureStorageKey, data: Data) throws {
		store[key.identifier] = data
	}
	public func deleteDataForKey(key: SecureStorageKey) throws {
		store.removeValue(forKey: key.identifier)
	}

}
extension EphemeralKeychain {
	public static let shared = EphemeralKeychain()

	public var isEmpty: Bool {
		store.isEmpty
	}
	public func contains(value: String) -> Bool {
		store
			.values
			.map({ String(data: $0, encoding: .utf8)! })
			.contains(where: { $0.contains(value) })

	}
}
