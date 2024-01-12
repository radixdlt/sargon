import Foundation
import radix_wallet_kit

extension Profile {
	fileprivate static let placeholder = newProfilePlaceholder()
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

extension SecureStorageKey {
	var identifier: String {
		secureStorageKeyIdentifier(key: self)
	}
}

public final class EphemeralKeychain: SecureStorage {
	private var store: [String: String]
	private init() {
		store = [:]
	}
	public static let shared = EphemeralKeychain()
	public func loadData(key: SecureStorageKey) throws -> Data? {
		store[key.identifier]
	}
	public func saveData(key: SecureStorageKey, value: Data) throws {
		store[key.identifier] = value
	}
}

func test() throws {
	print("🚀 Test Wallet in Swift start")
	defer { print("✅ Test Wallet in Swift completed ") }
	let secureStorage = EphemeralKeychain.shared

	let privateHDFactorSource = try newPrivateHdFactorSource(
		entropy: Data.random(byteCount: 32),
		walletClientModel: .iphone
	)
	let profile = newProfile(
		privateHdFactorSource: privateHDFactorSource,
		creatingDeviceName: "IntegrationTest"
	)
	let wallet = Wallet(
		profile: profile,
		secureStorage: secureStorage
	)
	wallet.createNew

	do {
		let profile = wallet.profile()
		assert(profile.networks.count > 1)
		/*
		let mainnet = profile.networks[0]
		assert(mainnet.id == .mainnet)
		let mainnetAccounts = mainnet.accounts
		assert(mainnetAccounts.count > 1)
		let account = mainnetAccounts[0]
		assert(account.displayName.value == "Alice")
		let address = account.address
		assert(
			address.address
				== "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
		)
		let newName = try newDisplayName(name: "Satoshi")

		let renamed = try wallet.changeNameOfAccount(
			address: address, to: newName
		)
		assert(
			renamed
				.displayName.value == "Satoshi")

		assert(account.displayName.value == "Alice")  // all types are VALUE types, so the prev `let` variable should NOT have been changed (which would be the case if we used classes...)
*/
	} catch {
		print("Failed to do stuff ❌ error: \(error)")
		return
	}
	/*
	do {
		let profile = wallet.profile()
		let mainnet = profile.networks[0]
		let mainnetAccounts = mainnet.accounts
		let account = mainnetAccounts[0]
		assert(account.displayName.value == "Satoshi")
	}
*/
}

try! test()
