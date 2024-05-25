import Sargon
import Dependencies
import Foundation
import ComposableArchitecture

public typealias AccountsForDisplay = IdentifiedArrayOf<AccountForDisplay>
extension IdentifiedArray where Element: Identifiable, Element.ID == ID {
	public static var `default`: Self { IdentifiedArrayOf.init() }
}
extension NetworkID {
	public static let `default` = Self.mainnet
}

extension SargonOS {
	
	public var accountsForDisplayOnCurrentNetworkIdentified: AccountsForDisplay {
		accountsForDisplayOnCurrentNetwork.asIdentified()
	}
	
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<AccountsForDisplay>> {
	public static var accountsForDisplay: Self {
		Self.sharedAccountsForDisplay
	}
}

extension PersistenceKeyDefault<SargonKey<AccountsForDisplay>> {
	public static let sharedAccountsForDisplay = Self(
		SargonKey(
			accessing: \.accountsForDisplayOnCurrentNetworkIdentified,
			fetchIf: \.affectsCurrentAccounts
		),
		.default
	)
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static var network: Self {
		Self.sharedNetwork
	}
}

extension PersistenceKeyDefault: @unchecked Sendable {}
extension PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static let sharedNetwork = Self(
		SargonKey(
			accessing: \.currentNetworkID,
			fetchIf: \.affectsCurrentNetwork
		),
		.default
	)
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static var savedGateways: Self {
		Self.sharedSavedGateways
	}
}

extension PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static let sharedSavedGateways = Self(
		SargonKey(
			accessing: \.gateways,
			fetchIf: \.affectsSavedGateways
		),
			.default
	)
	
}


public struct SargonKey<Value>: Hashable, PersistenceReaderKey, Sendable {
	public typealias LastValue = @Sendable () -> Value?
	public typealias ShouldFetch = @Sendable (EventKind) -> Bool

	private let lastValue: LastValue
	private let shouldFetch: ShouldFetch
	
	public init(
		lastValueWithOS: @escaping @Sendable (SargonOS) -> Value?,
		shouldFetch: @escaping ShouldFetch
	) {
		self.lastValue = {
			lastValueWithOS(SargonOS.shared)
		}
		self.shouldFetch = shouldFetch
		log.warning("SharedState for \(String(describing: Value.self)), hopefully just one per value kind")
	}
}

extension SargonKey {
	public init(
		accessing lastValueWithOSKeyPath: KeyPath<SargonOS, Value>,
		fetchIf fetchIfKeyPath: KeyPath<EventKind, Bool>
	) {
		self.init(
			lastValueWithOS: { $0[keyPath: lastValueWithOSKeyPath] },
			shouldFetch: { $0[keyPath: fetchIfKeyPath] }
		)

	}
}

extension AnyKeyPath: @unchecked Sendable {}


// MARK: PersistenceReaderKey
extension SargonKey {
	public func load(initialValue: Value?) -> Value? {
		lastValue() ?? initialValue
	}
	
	public func subscribe(
	  initialValue: Value?,
	  didSet: @Sendable @escaping (_ newValue: Value?) -> Void
	) -> Shared<Value>.Subscription {
		let task = Task { [shouldFetch = self.shouldFetch] in
			for await _ in await EventBus.shared.notifications().map(\.event.kind).filter({
				shouldFetch($0)
			}) {
				guard !Task.isCancelled else { return }
				didSet(lastValue())
			}
		}
		return .init {
			task.cancel()
		}
	}
}

extension SargonKey {
	private var valueKind: String {
		String(describing: Value.self)
	}
}

// MARK: Equatable
extension SargonKey {
	public static func == (lhs: SargonKey<Value>, rhs: SargonKey<Value>) -> Bool {
		lhs.valueKind == rhs.valueKind && EventKind.allCases.map(lhs.shouldFetch) == EventKind.allCases.map(rhs.shouldFetch)
	}
}

// MARK: Hashable
extension SargonKey {
	public func hash(into hasher: inout Hasher) {
		hasher.combine(valueKind)
		hasher.combine(EventKind.allCases.map(shouldFetch))
	}
}
