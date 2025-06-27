import ComposableArchitecture
import Sargon
import SwiftUI

// MARK: - OnboardingFeature
@Reducer
public struct OnboardingFeature {
	@Reducer(state: .equatable)
	public enum Path {
		case newOrImportProfile(NewOrImportProfileFeature)
		case writeDownMnemonic(WriteDownMnemonicFeature)
	}

	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
		case importProfile(ImportProfileFlowFeature)
	}

	@ObservableState
	public struct State: Equatable {
		public var path = StackState<Path.State>()
		public var welcome: WelcomeFeature.State

		@Presents var destination: Destination.State?

		public init() {
			self.welcome = WelcomeFeature.State()
		}
	}

	@CasePathable
	public enum Action {
		@CasePathable
		public enum DelegateAction {
			case done
		}

		case destination(PresentationAction<Destination.Action>)
		case path(StackAction<Path.State, Path.Action>)
		case welcome(WelcomeFeature.Action)
		case delegate(DelegateAction)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Scope(state: \.welcome, action: \.welcome) {
			WelcomeFeature()
		}
		Reduce { state, action in
			switch action {
			case let .path(pathAction):
				switch pathAction {
				case .element(id: _, action: .writeDownMnemonic(.delegate(.done))):
					return .send(.delegate(.done))

				case .element(id: _, action: .newOrImportProfile(.delegate(.importProfile))):
					state.destination = .importProfile(.init())
					return .none

				case .element(id: _, action: .newOrImportProfile(.delegate(.createdNewEmptyProfile))):
					state.destination = .createAccount(CreateAccountFlowFeature.State(index: 0))
					return .none

				case .popFrom(id: _):
					return .none

				case .push(id: _, state: _):
					return .none

				default:
					return .none
				}

			case .welcome(.delegate(.done)):
				state.path.append(.newOrImportProfile(.init()))
				return .none

			case .welcome:
				return .none

			case .delegate:
				return .none

			case .destination(.presented(.importProfile(.delegate(.imported)))):
				state.destination = nil
				return .send(.delegate(.done))

			case .destination(.presented(.createAccount(.delegate(.createdAccount)))):
				state.destination = nil
				state.path.append(.writeDownMnemonic(.init()))
				return .none

			default:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
		.ifLet(\.$destination, action: \.destination)
	}
}

// MARK: OnboardingFeature.View
extension OnboardingFeature {
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<OnboardingFeature>

		public init(store: StoreOf<OnboardingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				WelcomeFeature.View(
					store: store.scope(state: \.welcome, action: \.welcome)
				)
			} destination: { store in
				switch store.case {
				case let .newOrImportProfile(store):
					NewOrImportProfileFeature.View(store: store)
				case let .writeDownMnemonic(store):
					WriteDownMnemonicFeature.View(store: store)
				}
			}
			.sheet(
				item: $store.scope(state: \.destination?.createAccount, action: \.destination.createAccount)
			) { store in
				CreateAccountFlowFeature.View(store: store)
			}
			.sheet(
				item: $store.scope(state: \.destination?.importProfile, action: \.destination.importProfile)
			) { importProfileStore in
				ImportProfileFlowFeature.View(store: importProfileStore)
			}
		}
	}
}
