//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-06.
//

import Foundation
import Sargon
import ComposableArchitecture

@Reducer
public struct PrimaryRoleFactorsFeature {

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.factorSources) var allInProfile
		
		var available: FactorSources {
			let idsOfAllPicked = idsOfAllPicked()
			return allInProfile.filter({ !idsOfAllPicked.contains($0.id) }).asIdentified()
		}
		
		@Shared(.thresholdFactors) var thresholdFactors = [.factor(.sample)]
		@Shared(.overrideFactors) var overrideFactors = []

		public var threshold: FactorThreshold = .any
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
			case pickButtonTapped
			case thresholdFactorsChanged(Factors)
			case overrideFactorsChanged(Factors)
		}
		
		public enum DelegateAction {
			case `continue`
			case pickFactor
		}
		
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
				
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue))
				
			case .view(.pickButtonTapped):
				return .send(.delegate(.pickFactor))
				
			case let .view(.thresholdFactorsChanged(new)):
				state.thresholdFactors = new
				return .none
				
			case let .view(.overrideFactorsChanged(new)):
				state.overrideFactors = new
				return .none
				
			case .delegate:
				return .none
			}
		}
	}
}

extension PrimaryRoleFactorsFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			ScrollView {
				VStack(alignment: .center, spacing: 10) {
					Text("Sign Transactions").font(.largeTitle)
					
					Text("These factors are required to withdraw your assets and log in to dApps.")
						.foregroundStyle(Color.app.gray4)
					
					FactorsBuilderView(
						factors: $store.thresholdFactors.sending(\.view.thresholdFactorsChanged),
						factorThreshold: store.threshold,
						title: "Threshold Factors",
						titleAction: {
							log.info("Threshold factors rule!")
						},
						changeThresholdAction: {
							log.info("TODO change threshold")
						},
						pickAction: {
							send(.pickButtonTapped)
						}
					)
					
					FactorsBuilderView(
						factors: $store.overrideFactors.sending(\.view.overrideFactorsChanged),
						factorThreshold: .any,
						title: "Override Factors",
						titleAction: {
							log.info("Override factors are good.")
						},
						changeThresholdAction: nil,
						pickAction: {
							send(.pickButtonTapped)
						}
					)
					
					Button("Confirm") {
						send(.confirmButtonTapped)
					}
					.buttonStyle(.borderedProminent)
				}
				.padding()
			}
		}
	}
}




#Preview {
	PrimaryRoleFactorsFeature.View(
		store: Store(
			initialState: PrimaryRoleFactorsFeature.State(),
			reducer: {
				PrimaryRoleFactorsFeature()
			}
		)
	)
}
