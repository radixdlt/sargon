import Foundation
import ComposableArchitecture
import Sargon

extension PersistenceKeyDefault: @unchecked Sendable where Base: Sendable {}
extension KeyPath: @unchecked Sendable where Root: Sendable, Value: Sendable {}

/// A `PersistenceReaderKey` that listens to relevant event notifications emitted
/// by the SargonOS to which `SargonKey` subscribes using an `EventBus` and
/// based on `shouldFetch` we fetch the latest `Value` from the SargonOS
/// using `fetchValueFromSargonOS` method. For example we
/// might create a SharedReader which subscribs the active Profiles
/// non hidden accounts on the current network like so:
///
///	```
///	SargonKey(
///		fetchValueFromSargonOS: { os in os[keyPath: \.accountsForDisplayOnCurrentNetworkIdentified] },
///		shouldFetch: { eventKind in eventKind[keyPath: \.affectsCurrentAccounts] }
///	)
///	```
///
/// Which we also can create using `SargonKey:accessing:fetchIf` init, like so:
/// ```
/// SargonKey(
///		accessing: \.accountsForDisplayOnCurrentNetworkIdentified,
///		fetchIf: \.affectsCurrentAccounts
///	)
/// ```
public struct SargonKey<Value: Sendable>: Hashable, PersistenceReaderKey, Sendable {
    public typealias FetchValueFromSargonOS = @Sendable () -> Value?
    public typealias ShouldFetch = @Sendable (EventKind) -> Bool
    
    /// A closure which we invoke if `shouldFetch` returns `true` for a received `EventNotification`,
    /// which fetches a new value from SargonOS. The closure has already been translated from a `(SargonOS) -> Value?`
    /// closure in the initializer of `SargonKey` into a `() -> Value?`.
    private let fetchValueFromSargonOS: FetchValueFromSargonOS
    
    /// A predicate which returns `true` given an received `EventNotification` of a certain kind
    /// (`EventKind`) if we should call `fetchValueFromSargonOS` else `false`, if the
    /// event kind is not relevant for the `Value` of this `SargonKey`. E.g. the EventKind `.addedAccount`,
    /// does not affect the current network, so a `SargonKey<NetworkID>` should return `false` for
    /// a received `EventNotification` of kind `.addedAccount`.
    ///
    /// However, we SHOULD not make our own decisions here on the Swift side if a certain event is
    /// relevant or not, better to use the functions `affectsX` which we have wrapped as computed
    /// properties on `EventKind`, e.g. `eventKind.affectsCurrentAccounts`.
    private let shouldFetch: ShouldFetch
    
    /// Owned (retained) by SargonOS
    private unowned let eventBus: EventBus
    
    public init(
        sargonOS: SargonOS = .shared,
        eventBus: EventBus = .shared,
        fetchValueFromSargonOS: @escaping @Sendable (SargonOS) throws -> Value?,
        shouldFetch: @escaping ShouldFetch
    ) {
        self.eventBus = eventBus
        self.fetchValueFromSargonOS = { [weak sargonOS] in
            guard let sargonOS else {
                return nil
            }
            return try? fetchValueFromSargonOS(sargonOS)
        }
        self.shouldFetch = shouldFetch
    }
}


extension SargonKey {
    
    /// Create a new `SargonKey` with `KeyPath` based API, instead of closure based.
    ///
    /// This allows use to write:
    ///
    /// ```
    /// SargonKey(
    ///		accessing: \.accountsForDisplayOnCurrentNetworkIdentified,
    ///		fetchIf: \.affectsCurrentAccounts
    ///	)
    /// ```
    ///
    /// Instead of more verbose:
    ///
    /// ```
    ///	SargonKey(
    ///		fetchValueFromSargonOS: { os in os[keyPath: \.accountsForDisplayOnCurrentNetworkIdentified] },
    ///		shouldFetch: { eventKind in eventKind[keyPath: \.affectsCurrentAccounts] }
    ///	)
    ///	```
    public init(
        accessing lastValueWithOSKeyPath: KeyPath<SargonOS, Value>,
        fetchIf fetchIfKeyPath: KeyPath<EventKind, Bool>
    ) {
        self.init(
            fetchValueFromSargonOS: { $0[keyPath: lastValueWithOSKeyPath] },
            shouldFetch: { $0[keyPath: fetchIfKeyPath] }
        )
    }
    
    public init(
        mapping fetchValueFromSargonOS: @escaping @Sendable (SargonOS) throws -> Value?,
        fetchIf fetchIfKeyPath: KeyPath<EventKind, Bool>
    ) {
        self.init(
            fetchValueFromSargonOS: fetchValueFromSargonOS,
            shouldFetch: { $0[keyPath: fetchIfKeyPath] }
        )
    }
}

// MARK: PersistenceReaderKey
extension SargonKey {
    
    /// Loads the freshest value from storage (SargonOS). Returns `nil` if there is no value in storage.
    ///
    /// - Parameter initialValue: An initial value assigned to the `@Shared` property.
    /// - Returns: An initial value provided by an external system, or `nil`.
    public func load(initialValue: Value?) -> Value? {
        fetchValueFromSargonOS() ?? initialValue
    }
    
    
    /// Subscribes to external updates, we do it by subscribing to `EventBus.notifications()`.
    ///
    /// - Parameters:
    ///   - initialValue: An initial value assigned to the `@Shared` property.
    ///   - didSet: A closure that is invoked with new values from an external system, or `nil` if the
    ///     external system no longer holds a value.
    /// - Returns: A subscription to updates from an external system. If it is cancelled or
    ///   deinitialized, the `didSet` closure will no longer be invoked.
    public func subscribe(
        initialValue: Value?,
        didSet: @Sendable @escaping (_ newValue: Value?) -> Void
    ) -> Shared<Value>.Subscription {
        let task = Task { [shouldFetch = self.shouldFetch] in
            for await _ in await eventBus.notifications().map(\.event.kind).filter({
                shouldFetch($0)
            }) {
                guard !Task.isCancelled else { return }
                
                // The call `fetchValueFromSargonOS` might be costly
                // we SHOULD try to use as fast and cheap calls as possible
                // i.e. it is best to call `os.currentNetwork()` which is near instant
                // compared to `os.profile().gateways.current.network.id` which is
                // costly, since the whole of Profile has to pass across the UniFFI
                // boundary
                let newValue = fetchValueFromSargonOS()
                
                didSet(newValue)
            }
        }
        return .init {
            task.cancel()
        }
    }
}

extension SargonKey {
    /// A String representation of `Self.Value`, used for `Equatable` and `Hashable`
    /// conformance.
    private var valueKind: String {
        String(describing: Value.self)
    }
}

// MARK: Equatable
extension SargonKey {
    public static func == (lhs: SargonKey<Value>, rhs: SargonKey<Value>) -> Bool {
        lhs.valueKind == rhs.valueKind && /* this aint pretty, but I guess it works */ EventKind.allCases.map(lhs.shouldFetch) == EventKind.allCases.map(rhs.shouldFetch)
    }
}

// MARK: Hashable
extension SargonKey {
    public func hash(into hasher: inout Hasher) {
        hasher.combine(valueKind)
        /* this aint pretty, but I guess it works */
        hasher.combine(EventKind.allCases.map(shouldFetch))
    }
}
