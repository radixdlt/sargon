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
		public var accountForDisplay: AccountForDisplay
		
		@Presents var destination: Destination.State?
	
		public init(accountForDisplay: AccountForDisplay) {
			self.accountForDisplay = accountForDisplay
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
				state.destination = .changeGradient(
					SelectGradientFeature.State(gradient: state.accountForDisplay.appearanceId)
				)
				return .none

			case let .destination(.presented(.changeGradient(.delegate(.selected(newGradient))))):
				state.destination = nil
				
				return .run { [address = state.accountForDisplay.address] send in
					var account = try accountsClient.accountByAddress(address)
					account.appearanceId = newGradient
					try await accountsClient.updateAccount(account)
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
					AccountView(accountForDisplay: store.accountForDisplay, format: .full)
					
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

