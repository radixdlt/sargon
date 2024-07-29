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
public struct RoleFactorsFeature {
	
	
	@Reducer(state: .equatable)
	public enum Destination {
		case pickFactorSourceCoordinator(PickFactorSourceCoordinator)
		case setFactorThreshold(SetFactorThresholdFeature)
		case setDaysUntilAutoConfirm(SetDaysUntilAutoConfirm)
	}

	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft
		public var thresholdFactorsBuilder: FactorsBuilderFeature.State
		public var overrideFactorsBuilder: FactorsBuilderFeature.State
		public let role: Role
		
		@Presents var destination: Destination.State?
		
		public var daysUntilAutoConfirm: UInt16 {
			newShieldDraft.numberOfDaysUntilAutoConfirmation
		}
		
		public init(role: Role) {
			self.role = role
			self.thresholdFactorsBuilder = FactorsBuilderFeature.State(mode: .threshold, role: role)
			self.overrideFactorsBuilder = FactorsBuilderFeature.State(mode: .override, role: role)
		}
		
		var canProceed: Bool {
			newShieldDraft.isValidRole(role)
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
			case changeDaysUntilAutoConfirmButtonTapped
        }
        
        public enum DelegateAction {
			case `continue`(role: Role)
        }
		
		case destination(PresentationAction<Destination.Action>)
		
        
        case view(ViewAction)
		case delegate(DelegateAction)
		
		case thresholdFactorsBuilder(FactorsBuilderFeature.Action)
		case overrideFactorsBuilder(FactorsBuilderFeature.Action)
		
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.thresholdFactorsBuilder, action: \.thresholdFactorsBuilder) {
			FactorsBuilderFeature()
		}
		Scope(state: \.overrideFactorsBuilder, action: \.overrideFactorsBuilder) {
			FactorsBuilderFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .view(.changeDaysUntilAutoConfirmButtonTapped):
				state.destination = .setDaysUntilAutoConfirm(SetDaysUntilAutoConfirm.State())
				return .none
				
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue(role: state.role)))
                
			case .thresholdFactorsBuilder(.delegate(.pickFactor)), .overrideFactorsBuilder(.delegate(.pickFactor)):
				state.destination = .pickFactorSourceCoordinator(PickFactorSourceCoordinator.State(role: state.role))
				return .none
				
				
			case .thresholdFactorsBuilder(.delegate(.setThreshold)):
				state.destination = .setFactorThreshold(SetFactorThresholdFeature.State(
					role: state.role
				))
				return .none
				

			case .destination(.presented(.setFactorThreshold(.delegate(.confirm)))):
				state.destination = nil
				return .none
				
			case .destination(.presented(.pickFactorSourceCoordinator(.delegate(.done)))):
				state.destination = nil
				return .none
				
			case .destination(.presented(.setDaysUntilAutoConfirm(.delegate(.done)))):
				state.destination = nil
				return .none

			case .thresholdFactorsBuilder:
				return .none
		
			case .overrideFactorsBuilder:
				return .none
				
			case .destination:
				return .none

			case .delegate:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
}

extension RoleFactorsFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			ScrollView {
				VStack(alignment: .center, spacing: 20) {
					Text("\(store.role.title)").font(.largeTitle)
					
					Text("These factors are required to \(store.role.actionDetailed)")
						.foregroundStyle(Color.app.gray2)
					
					FactorsBuilderFeature.View(
						store: store.scope(
							state: \.thresholdFactorsBuilder,
							action: \.thresholdFactorsBuilder
						)
					)
					
					FactorsBuilderFeature.View(
						store: store.scope(
							state: \.overrideFactorsBuilder,
							action: \.overrideFactorsBuilder
						)
					)

					if store.role == .recovery {
						Button(action: {
							send(.changeDaysUntilAutoConfirmButtonTapped)
						}, label: {
							HStack {
								Image(systemName: "lock")
								VStack {
									Text("Wallet Lock")
									Text("Lock duration")
										.font(.system(size: 14))
										.foregroundStyle(Color.app.gray3)
								}
								Spacer()
								Text("\(store.daysUntilAutoConfirm) days")
									.fontWeight(.bold)
									.foregroundStyle(Color.app.blue2)
							}
							.padding()
							.multilineTextAlignment(.leading)
							.foregroundStyle(Color.app.gray1)
							.overlay(
								RoundedRectangle(cornerRadius: 15)
									.inset(by: 1)
									.stroke(.gray, lineWidth: 1)
							)
							.frame(maxWidth: .infinity)
							.padding()
						})
						.buttonStyle(.plain)
						.frame(maxWidth: .infinity)
					}
					
					Button("Confirm") {
						send(.confirmButtonTapped)
					}
					.buttonStyle(.borderedProminent)
					.disabled(!store.canProceed)
				}
				.padding()
			}
			.sheet(
				item: $store.scope(state: \.destination?.setDaysUntilAutoConfirm, action: \.destination.setDaysUntilAutoConfirm)
			) { store in
				SetDaysUntilAutoConfirm.View(store: store)
			}
			.sheet(
				item: $store.scope(state: \.destination?.pickFactorSourceCoordinator, action: \.destination.pickFactorSourceCoordinator)
			) { store in
				PickFactorSourceCoordinator.View(store: store)
			}
			.sheet(
				item: $store.scope(state: \.destination?.setFactorThreshold, action: \.destination.setFactorThreshold)
			) { store in
				SetFactorThresholdFeature.View(store: store)
					.presentationDetents([.medium])
			}
		}
	}
}




#Preview {
	RoleFactorsFeature.View(
		store: Store(
			initialState: RoleFactorsFeature.State(role: .primary),
			reducer: {
				RoleFactorsFeature()
			}
		)
	)
}
