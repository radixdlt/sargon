import Foundation
import SargonUniFFI

extension EventProfileModified {
	public var addedAccount: AccountAddress? {
		guard case let .accountAdded(address) = self else { return nil }
		return address
	}
}

extension Event {
	public var profileModified: EventProfileModified? {
		switch self {
		case let .profileModified(change): change
		default: nil
		}
	}

	public var addressOfNewAccount: AccountAddress? {
		profileModified?.addedAccount
	}
}

// MARK: - EventNotification + Comparable
extension EventNotification: Comparable {
	/// `EventNotification` are made `Comparable` by
	/// sorting on `timestamp`.
	public static func < (lhs: Self, rhs: Self) -> Bool {
		lhs.timestamp < rhs.timestamp
	}
}

extension Event {
	/// Discriminant of the `Event`.
	public var kind: EventKind {
		eventKind(event: self)
	}
}
