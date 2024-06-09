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
public struct NewSecurityShieldCoordinator {
	
	@Reducer(state: .equatable)
	public enum Destination {
		case pickFactorSourceCoordinator(PickFactorSourceCoordinator)
		case setFactorThreshold(SetFactorThresholdFeature)
	}
	
	@Reducer(state: .equatable)
	public enum Path {
		case roleFactors(RoleFactorsFeature)
		case nameShield(NameNewShieldFeature)
	}
	
	
	@ObservableState
	public struct State: Equatable {
		
		public var intro: IntroWhatIsShieldFeature.State
		public var path = StackState<Path.State>()
	
		@Presents var destination: Destination.State?
		
		public let preset: Shield?
		public init(preset: Shield?) {
			self.preset = preset
			self.intro = IntroWhatIsShieldFeature.State()
		}
	}
	
	@CasePathable
	public enum Action {
		case path(StackAction<Path.State, Path.Action>)
		case intro(IntroWhatIsShieldFeature.Action)
	
		case destination(PresentationAction<Destination.Action>)
		case delegate(DelegateAction)
		
		public enum DelegateAction {
			case done
		}
	}

	
	private func next(lastRole: Role? = nil, _ state: inout State) -> EffectOf<Self> {
		let nextRole: Role? = switch lastRole {
		case .none:
				.primary
		case .primary:
				.recovery
		case .recovery:
				.confirmation
		case .confirmation:
			nil
		}
		if let nextRole  {
			state.path.append(.roleFactors(RoleFactorsFeature.State(role: nextRole)))
		} else {
			state.path.append(.nameShield(NameNewShieldFeature.State()))
		}
		return .none
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.intro, action: \.intro) {
			IntroWhatIsShieldFeature()
		}
		Reduce { state, action in
			switch action {
			case .intro(.delegate(.continue)):
				return next(&state)
				
			case let .path(.element(id: _, action: .roleFactors(.delegate(.pickFactor(role))))):
				state.destination = .pickFactorSourceCoordinator(PickFactorSourceCoordinator.State(
					role: role
				))
				return .none
				
			case let .path(.element(id: _, action: .roleFactors(.delegate(.setThreshold(role))))):
				state.destination = .setFactorThreshold(SetFactorThresholdFeature.State(
					role: role
				))
				return .none
				
			case let .path(.element(id: _, action: .roleFactors(.delegate(.continue(role))))):
				return next(lastRole: role, &state)
				
			case .path(.element(id: _, action: .nameShield(.delegate(.done)))):
				return .send(.delegate(.done))
				
			case .path:
				return .none
                
			case .destination(.presented(.setFactorThreshold(.delegate(.confirm)))):
				state.destination = nil
				return .none
				
			case .destination(.presented(.pickFactorSourceCoordinator(.delegate(.done)))):
                state.destination = nil
                return .none

            case .destination:
                return .none
				
			case .intro:
				return .none
			case .delegate:
				return .none
				
			}
		}
		.forEach(\.path, action: \.path)
		.ifLet(\.$destination, action: \.destination)
	}
}

extension NewSecurityShieldCoordinator {
	public typealias HostingFeature = Self
	
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				IntroWhatIsShieldFeature.View(
					store: store.scope(state: \.intro, action: \.intro)
				)
			} destination: { store in
				switch store.case {
				case let .roleFactors(store):
					RoleFactorsFeature.View(store: store)
				case let .nameShield(store):
					NameNewShieldFeature.View(store: store)
				}
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
