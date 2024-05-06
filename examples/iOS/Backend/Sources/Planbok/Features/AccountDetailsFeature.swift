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
	
	@Dependency(AccountsClient.self) var accountsClient
	
	@Reducer(state: .equatable)
	public enum Destination {
		case changeGradient(SelectGradientFeature)
	}
	
	@ObservableState
	public struct State: Equatable {
		public var account: Account
		
		@Presents var destination: Destination.State?
	
		public init(account: Account) {
			self.account = account
		}
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case changeGradientButtonTapped
		}
		
		case destination(PresentationAction<Destination.Action>)
		case view(ViewAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.changeGradientButtonTapped):
				state.destination = .changeGradient(SelectGradientFeature.State.init(gradient: state.account.appearanceID))
				return .none

			case let .destination(.presented(.changeGradient(.delegate(.selected(newGradient))))):
				state.destination = nil
				state.account.appearanceId = newGradient
				
				return .run { [updatedAccount = state.account] send in
					try await accountsClient.updateAccount(updatedAccount)
				}
				
			default:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
}

extension AccountDetailsFeature {
	
	@ViewAction(for: AccountDetailsFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<AccountDetailsFeature>
		public var body: some SwiftUI.View {
			NavigationView {
				VStack {
					AccountView(account: store.account, format: .full)
					
					Button("Change Gradient") {
						send(.changeGradientButtonTapped)
					}
				}
			}
			.sheet(
				item: $store.scope(state: \.destination?.changeGradient, action: \.destination.changeGradient)
			) { store in
				SelectGradientFeature.View(store: store)
			}
		}
	}
}

