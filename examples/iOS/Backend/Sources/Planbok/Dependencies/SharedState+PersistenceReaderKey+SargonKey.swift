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
		Self(
			SargonKey(
				on: .currentAccounts,
				accessing: \.accountsForDisplayOnCurrentNetworkIdentified
			),
			AccountsForDisplay.default
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static var network: Self {
		Self(
			SargonKey(
				on: .currentGateway,
				accessing: \.currentNetworkID
			),
			NetworkID.default
		)
	}
}

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static var savedGateways: Self {
		Self(
			SargonKey(
				on: .currentGateway,
				accessing: \.gateways
			),
			SavedGateways.default
		)
	}
}

extension Set<EventKind> {
	public static let currentGateway: Self = [.booted, .gatewayChangedCurrent]
	public static let currentAccounts: Self = [.booted, .addedAccount, .addedAccounts, .gatewayChangedCurrent]
}

public struct SargonKey<Value>: Equatable, PersistenceReaderKey, Sendable {
	public typealias LastValue = @Sendable () -> Value?

	private let lastValue: LastValue
	private let fetchOnEvents: Set<EventKind>
	
	public init(
		on fetchOnEvents: Set<EventKind>,
		lastValueWithOS: @escaping @Sendable (SargonOS) -> Value?
	) {
		self.fetchOnEvents = fetchOnEvents
		self.lastValue = {
			lastValueWithOS(SargonOS.shared)
		}
	}
}

extension SargonKey {
	public init(
		on fetchOnEvents: Set<EventKind>,
		accessing keyPath: KeyPath<SargonOS, Value>
	) {
		self.init(on: fetchOnEvents, lastValueWithOS: { $0[keyPath: keyPath] })
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
		let task = Task { [fetchOnEvents = self.fetchOnEvents] in
			for await _ in await EventBus.shared.notifications().map(\.event.kind).filter({
				fetchOnEvents.contains($0)
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
		lhs.valueKind == rhs.valueKind && lhs.fetchOnEvents == rhs.fetchOnEvents
	}
}

// MARK: Hashable
extension SargonKey {
	public func hash(into hasher: inout Hasher) {
		hasher.combine(valueKind)
		hasher.combine(fetchOnEvents)
	}
}
