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
public struct SettingsFeature {
	@Dependency(AccountsClient.self) var accountsClient
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createManyAccountsAlert(AlertState<CreateManyAccountsAlert>)
		
		public enum CreateManyAccountsAlert: Int, CaseIterable {
			case create10 = 10
			case create20 = 20
			case create50 = 50
			case create100 = 100
			case create200 = 200
			case create500 = 500
			case create1000 = 1000
		}
	}
	
	@ObservableState
	public struct State: Equatable {
		@Presents var destination: Destination.State?
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case createManyAccountsButtonTapped
		}
		
		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)
	}
	
	public init() {}
	
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.createManyAccountsButtonTapped):
				
				state.destination = .createManyAccountsAlert(.init(
					title: TextState("How many?"),
					message: TextState("Will batch create many accounts and then perform one single save action."),
					buttons: [
						.cancel(TextState("Cancel"))
					] + Destination.CreateManyAccountsAlert.allCases.map { action in
						ButtonState<Destination.CreateManyAccountsAlert>.init(action: action, label: {
							TextState("Create \(action.rawValue)")
						})
					}
				))
				return .none
				
			case let .destination(.presented(.createManyAccountsAlert(action))):
				state.destination = nil
				let count = UInt16(action.rawValue)
				return .run { send in
					try await accountsClient.batchCreateManySavedAccounts(count, NetworkID.mainnet)
				}
				
			default:
				return .none
				
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
}

extension SettingsFeature {
	
	@ViewAction(for: SettingsFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<SettingsFeature>
		public var body: some SwiftUI.View {
			VStack {
				Text("Settings").font(.largeTitle)
				Spacer()
				Button("Create Many Accounts") {
					send(.createManyAccountsButtonTapped)
				}
			}
			.padding(.bottom, 100)
			.alert($store.scope(state: \.destination?.createManyAccountsAlert, action: \.destination.createManyAccountsAlert))
		}
	}
	
}


