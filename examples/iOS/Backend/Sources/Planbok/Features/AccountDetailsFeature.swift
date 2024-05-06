//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct AccountDetailsFeature {
	
	@ObservableState
	public struct State: Equatable {
		public let account: Account
		public init(account: Account) {
			self.account = account
		}
	}
	
	public enum Action {}
	
	public init() {}
}

extension AccountDetailsFeature {
	public struct View: SwiftUI.View {
		public let store: StoreOf<AccountDetailsFeature>
		public var body: some SwiftUI.View {
			VStack {
				Text("TODO AccountDetails")
				Text("Account: \(store.account.displayName)")
				Text("Address: \(store.account.address)")
			}
		}
	}
}

