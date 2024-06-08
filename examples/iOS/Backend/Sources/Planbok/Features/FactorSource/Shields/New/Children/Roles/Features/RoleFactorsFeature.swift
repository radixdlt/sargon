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
		
		public var thresholdFactorsBuilder: FactorsBuilderFeature.State
		public var overrideFactorsBuilder: FactorsBuilderFeature.State
		
		public let role: Role
		public init(role: Role) {
			self.role = role
			self.thresholdFactorsBuilder = FactorsBuilderFeature.State(listKind: .threshold, role: role)
			self.overrideFactorsBuilder = FactorsBuilderFeature.State(listKind: .override, role: role)
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


		public var pickedFactorID: Factor.ID? {
			get {
				__newShieldDraft.pendingFactorID
			}
			set {
				__newShieldDraft.pendingFactorID = newValue
			}
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
        }
        
        public enum DelegateAction {
			case `continue`(role: Role)
			
			case pickFactor(role: Role)
			case setThreshold(role: Role)
        }
		
        
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
				
				
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue(role: state.role)))
                
			case .thresholdFactorsBuilder(.delegate(.pickFactor(let role))), .overrideFactorsBuilder(.delegate(.pickFactor(let role))):
				return .send(.delegate(.pickFactor(role: role)))
				
			case let .thresholdFactorsBuilder(.delegate(.setThreshold(role))):
				return .send(.delegate(.setThreshold(role: role)))
				
			case .thresholdFactorsBuilder:
				return .none
			case .overrideFactorsBuilder:
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
					
					FactorsBuilderFeature.View(store: store.scope(state: \.thresholdFactorsBuilder, action: \.thresholdFactorsBuilder))
					FactorsBuilderFeature.View(store: store.scope(state: \.overrideFactorsBuilder, action: \.overrideFactorsBuilder))

					
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
	RoleFactorsFeature.View(
		store: Store(
			initialState: RoleFactorsFeature.State(role: .primary),
			reducer: {
				RoleFactorsFeature()
			}
		)
	)
}
