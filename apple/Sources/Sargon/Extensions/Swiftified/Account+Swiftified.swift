//
//  Account+Swiftified.swift
//
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

extension Account: @unchecked Sendable {}
extension Account: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		address
	}
}
extension Account {
	public var appearanceID: AppearanceID {
		appearanceId
	}
}
