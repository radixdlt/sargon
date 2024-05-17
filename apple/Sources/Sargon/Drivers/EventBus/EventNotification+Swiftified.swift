//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-15.
//

import Foundation
import SargonUniFFI

extension EventProfileModified {
	public var addedAccount: AccountAddress? {
		guard case let .addedAccount(address) = self else { return nil }
		return address
	}
}

extension Event {
	public var profileModified: EventProfileModified? {
		switch self {
		case let .modifiedProfile(change): return change
		default: return nil
		}
	}
	public var addressOfNewAccount: AccountAddress? {
		profileModified?.addedAccount
	}
}

extension EventNotification: Comparable {
	public static func < (lhs: Self, rhs: Self) -> Bool {
		lhs.timestamp < rhs.timestamp
	}
}

extension Event {
	public var kind: EventKind {
		eventKind(event: self)
	}
}
