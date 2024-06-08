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

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.factorSources) var allInProfile
        @Shared(.newShieldDraft) var __newShieldDraft

		public let role: Role
		public init(role: Role) {
			self.role = role
		}

		var available: FactorSources {
//			let idsOfAllPicked = idsOfAllPicked()
			return allInProfile//.filter({ !idsOfAllPicked.contains($0.id) }).asIdentified()
		}
		
		public var matrixOfFactorsForRole: NewShieldDraft.MatrixOfFactorsForRole {
			get { __newShieldDraft[role] }
			set {
				__newShieldDraft[role] = newValue
			}
		}
		public var threshold: FactorThreshold {
			get {
				matrixOfFactorsForRole.threshold
			}
			set {
				matrixOfFactorsForRole.threshold = newValue
			}
		}
		public var thresholdFactors: Factors {
			get {
				matrixOfFactorsForRole.thresholdFactors
			}
			set {
				matrixOfFactorsForRole.thresholdFactors = newValue
			}
		}
		public var overrideFactors: Factors {
			get {
				matrixOfFactorsForRole.overrideFactors
			}
			set {
				matrixOfFactorsForRole.overrideFactors = newValue
			}
		}
		public var pickedFactor: Factor? {
			get {
				__newShieldDraft.pendingFactor
			}
			set {
				__newShieldDraft.pendingFactor = newValue
			}
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
			case pickButtonTapped
			case changeThresholdButtonTapped
			case thresholdFactorsChanged(Factors)
			case overrideFactorsChanged(Factors)
            case onPickedFactorChanged(old: Factor?, new: Factor?)
        }
        
        public enum DelegateAction {
			case `continue`(role: Role)
			case pickFactor(role: Role)
			case setThreshold(role: Role)
        }
        
        case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
				
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue(role: state.role)))
				
			case .view(.pickButtonTapped):
				return .send(.delegate(.pickFactor(role: state.role)))
				
			case .view(.changeThresholdButtonTapped):
				return .send(.delegate(.setThreshold(
					role: state.role
				)))
				
			case let .view(.thresholdFactorsChanged(new)):
				state.thresholdFactors = new
				return .none
				
			case let .view(.overrideFactorsChanged(new)):
				state.overrideFactors = new
				return .none
                
            case let .view(.onPickedFactorChanged(old, new)):
                guard let old, let new else { return .none }
                if state.thresholdFactors.contains(old) {
                    state.thresholdFactors[id: new.id] = new
                } else if state.overrideFactors.contains(old) {
                    state.overrideFactors[id: new.id] = new
                }
                // dont forget to nil it!
                state.pickedFactor = nil
                return .none
                
            case .delegate:
                return .none
			}
		}
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
				VStack(alignment: .center, spacing: 10) {
					Text("\(store.role.title)").font(.largeTitle)
					
					Text("These factors are required to \(store.role.actionDetailed)")
						.foregroundStyle(Color.app.gray2)
					
					FactorsBuilderView(
						factors: $store.thresholdFactors.sending(\.view.thresholdFactorsChanged),
						factorThreshold: store.threshold,
						title: "Threshold Factors",
						titleAction: {
							log.info("Threshold factors rule!")
						},
						changeThresholdAction: {
							send(.changeThresholdButtonTapped)
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
                .onChange(of: store.pickedFactor) { (oldState: Factor?, newState: Factor?) in
                    send(.onPickedFactorChanged(old: oldState, new: newState))
                }
				.padding()
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
