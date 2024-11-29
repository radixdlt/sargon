import ComposableArchitecture
import Foundation
import Sargon

// MARK: - NewSecurityShieldCoordinator
@Reducer
public struct NewSecurityShieldCoordinator {
	@Reducer(state: .equatable)
	public enum Path {
		case roleFactors(RoleFactorsFeature)
		case nameShield(NameNewShieldFeature)
	}

	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft
		public var intro: IntroWhatIsShieldFeature.State
		public var path = StackState<Path.State>()
		public init(copyAndEdit preset: Shield?) {
			self.intro = IntroWhatIsShieldFeature.State()
			if let preset {
				newShieldDraft = .init(copyAndEdit: preset)

				// skip intro
				HostingFeature.next(&self)
			}
		}
	}

	@CasePathable
	public enum Action {
		case path(StackAction<Path.State, Path.Action>)
		case intro(IntroWhatIsShieldFeature.Action)

		case delegate(DelegateAction)

		public enum DelegateAction {
			case done
		}
	}

	@discardableResult
	fileprivate static func next(
		lastRole: Role? = nil,
		_ state: inout State
	) -> EffectOf<Self> {
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
		if let nextRole {
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
				Self.next(&state)

			case let .path(.element(id: _, action: .roleFactors(.delegate(.continue(role))))):
				Self.next(lastRole: role, &state)

			case .path(.element(id: _, action: .nameShield(.delegate(.done)))):
				.send(.delegate(.done))

			case .path:
				.none

			case .intro:
				.none

			case .delegate:
				.none
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
			} destination: { store in
				switch store.case {
				case let .roleFactors(store):
					RoleFactorsFeature.View(store: store)
				case let .nameShield(store):
					NameNewShieldFeature.View(store: store)
				}
			}
		}
	}
}
