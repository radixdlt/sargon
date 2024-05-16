import Sargon
import Dependencies
import Foundation

extension Profile {
	public var identifiedAccountsOnCurrentNetwork: Accounts {
		accounts().asIdentified()
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<Accounts>> {
	public static var accounts: Self {
		PersistenceKeyDefault(
			SargonKey(keyPath: \.identifiedAccountsOnCurrentNetwork),
			[]
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static var network: Self {
		PersistenceKeyDefault(
			SargonKey(keyPath: \.currentNetworkID),
			NetworkID.mainnet
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static var savedGateways: Self {
		PersistenceKeyDefault(
			SargonKey(keyPath: \.appPreferences.gateways),
			SavedGateways.preset
		)
	}
}

extension PersistenceReaderKey {
	public static func sargon<Value>(keyPath: KeyPath<Profile, Value>) -> Self
	where Self == SargonKey<Value> {
		SargonKey(keyPath: keyPath)
	}
}

extension AnyKeyPath: @unchecked Sendable {}
public struct SargonKey<Value>: Equatable, PersistenceReaderKey, Sendable {
	public static func == (lhs: SargonKey<Value>, rhs: SargonKey<Value>) -> Bool {
		lhs.keyPath == rhs.keyPath
	}
	public func hash(into hasher: inout Hasher) {
		hasher.combine(self.keyPath)
	}
	
	private func lastValue() -> Value? {
		let lastProfile = profileProvider()
		guard
			let value = lastProfile[keyPath: self.keyPath] as? Value
		else {
			return nil
		}
		return value
	}
	
	public func load(initialValue: Value?) -> Value? {
		lastValue() ?? initialValue
	}
	
	public func subscribe(
	  initialValue: Value?,
	  didSet: @Sendable @escaping (_ newValue: Value?) -> Void
	) -> Shared<Value>.Subscription {
		let task = Task {
			// FIXME: Multicast eventBus!
			for await _ in await EventBus.shared.notifications().filter({
				switch $0.event {
				case .profileChanged, .profileSaved:
					true
				default:
					false
				}
			}) {
				guard !Task.isCancelled else { return }
				didSet(lastValue())
			}
		}
		return .init {
			task.cancel()
		}
	}
	
	private let keyPath: AnyKeyPath
	private let profileProvider: @Sendable () -> Profile
	public init(keyPath: KeyPath<Profile, Value>) {
		@Dependency(ProfileClient.self) var profileClient
		self.keyPath = keyPath
		self.profileProvider = { profileClient.activeProfile() }
	}
}
