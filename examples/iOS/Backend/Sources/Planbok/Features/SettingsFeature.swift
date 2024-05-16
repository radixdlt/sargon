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
				Button("Create Many Accounts") {
					send(.createManyAccountsButtonTapped)
				}
			}
			.padding(.bottom, 100)
			.alert($store.scope(state: \.destination?.createManyAccountsAlert, action: \.destination.createManyAccountsAlert))
		}
	}
	
}


@Reducer
public struct GatewaysFeature {
	
	@Dependency(GatewaysClient.self) var gatewaysClient
	
	@ObservableState
	public struct State: Equatable {
		@SharedReader(.savedGateways) var savedGateways
	}
	
	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case gatewayTapped(Gateway, isCurrent: Bool)
		}
		case view(ViewAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.gatewayTapped(gateway, isCurrent)):
				if isCurrent {
					log.debug("Tapped \(gateway), but not switching since it is already current.")
					return .none
				} else {
					log.info("Switching network to \(gateway)")
					return .run { _ in
						try await gatewaysClient.switchGatewayTo(gateway)
					}
				}
			}
		}
	}
}
extension GatewaysFeature {
	@ViewAction(for: GatewaysFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<GatewaysFeature>
		public var body: some SwiftUI.View {
			VStack {
				Text("Saved gateways").font(.title)
			
				ScrollView {
					ForEach(store.state.savedGateways.all.sorted()) { gateway in
						let isCurrent = gateway == store.state.savedGateways.current
						VStack {
							GatewayView(gateway: gateway, isCurrent: isCurrent) {
								send(.gatewayTapped(gateway, isCurrent: isCurrent))
							}
							.padding(.bottom, 10)
						}
					}
				}
			}
			.padding([.leading, .trailing], 20)
		}
	}
}

extension Gateway: Comparable {
	public static func < (lhs: Self, rhs: Self) -> Bool {
		if lhs.networkID == .mainnet { return true }
		if rhs.networkID == .mainnet { return false }
		return lhs.networkID.rawValue < rhs.networkID.rawValue && lhs.url.absoluteString < rhs.url.absoluteString
	}
}

public struct GatewayView: SwiftUI.View {
	public let gateway: Gateway
	public let isCurrent: Bool
	public let action: () -> Void
	
	public var body: some SwiftUI.View {
		
		Button.init(action: action, label: {
			HStack {
				Text(isCurrent ? "✅" : "☑️").font(.title)
				VStack {
					Text("\(gateway.network.displayDescription)").font(.body)
					Text("\(gateway.networkID.toString())")
				}
			}
		})
		.buttonStyle(.plain)
		.frame(maxWidth: .infinity, alignment: .leading)
		.cornerRadius(.small1)
	}
}
