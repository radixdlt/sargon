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
            case factorSourcesButtonTapped
			case shieldsButtonTapped
		}
		
		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)
        
        
        @CasePathable
        public enum DelegateAction {
            case navigate(Navigate)
            
            @CasePathable
            public enum Navigate {
                case toFactorSources
				case toShields
            }
        }
        
        case delegate(DelegateAction)
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
                
            case .view(.factorSourcesButtonTapped):
                return .send(.delegate(.navigate(.toFactorSources)))
				
			case .view(.shieldsButtonTapped):
				return .send(.delegate(.navigate(.toShields)))
				
			case let .destination(.presented(.createManyAccountsAlert(action))):
				state.destination = nil
				let count = UInt16(action.rawValue)
				return .run { send in
					try await accountsClient.batchCreateManySavedAccounts(count)
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
				
                GatewaysFeature.View(
					store: Store(
						initialState: GatewaysFeature.State()
					) {
						GatewaysFeature()
					}
				)
				
                Spacer()
                
                Button("Handle Factor Sources") {
                    send(.factorSourcesButtonTapped)
                }
				
				Button("Handle Security Shields") {
					send(.shieldsButtonTapped)
				}
                
				VStack {
					Button("Create Many Accounts") {
						send(.createManyAccountsButtonTapped)
					}
					Label("This button is here, in Settings, and not on Home screen since we wanna test that the accounts on Home screen is updated when we dismiss Settings.", systemImage: "info.circle")
						.font(.footnote)
				}
			}
			.padding(.bottom, 100)
			.padding(.horizontal, 10)
			.alert($store.scope(state: \.destination?.createManyAccountsAlert, action: \.destination.createManyAccountsAlert))
		}
	}
	
}

