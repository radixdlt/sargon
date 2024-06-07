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
	public enum Path {
		case primaryRoleFactors(PrimaryRoleFactorsFeature)
	}
	
	
	@ObservableState
	public struct State: Equatable {
		public var intro: IntroWhatIsShieldFeature.State
		public var path = StackState<Path.State>()
	
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
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.intro, action: \.intro) {
			IntroWhatIsShieldFeature()
		}
		Reduce { state, action in
			switch action {
			case .intro(.delegate(.continue)):
				state.path.append(.primaryRoleFactors(PrimaryRoleFactorsFeature.State()))
				return .none
				
			case .path(.element(id: _, action: .primaryRoleFactors(.delegate(.continue)))):
				log.fault("IGNORED should have navigated to next screen")
				return .none

			case .path:
				return .none
				
			case .intro:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
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
				.buttonStyle(.borderedProminent)
			} destination: { store in
				switch store.case {
				case let .primaryRoleFactors(store):
					PrimaryRoleFactorsFeature.View(store: store)
				}
			}
			.buttonStyle(.plain)

		}
	}
}
