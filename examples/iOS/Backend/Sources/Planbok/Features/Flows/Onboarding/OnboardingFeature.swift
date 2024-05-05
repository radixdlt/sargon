import SwiftUI
import Sargon
import ComposableArchitecture

@Reducer
public struct OnboardingFeature {
	
	@Reducer(state: .equatable)
	public enum Path {
		case writeDownMnemonic(WriteDownMnemonicFeature)
	}
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
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
			case createdAccount
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
				
			case .path(let pathAction):
				switch pathAction {
				case .element(id: _, action: .writeDownMnemonic(.delegate(.done))):
					return .send(.delegate(.createdAccount))
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				default:
					return .none
				}
			case .welcome(.delegate(.done)):
				state.destination = .createAccount(CreateAccountFlowFeature.State())
				return .none
			case .welcome(.view):
				return .none
			case .delegate:
				return .none
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

extension OnboardingFeature {
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<OnboardingFeature>
		
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
				case .writeDownMnemonic:
					if let store = store.scope(state: \.writeDownMnemonic, action: \.writeDownMnemonic) {
						WriteDownMnemonicFeature.View(store: store)
					}
				}
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.createAccount,
					action: \.destination.createAccount
				)
			) { store in
				CreateAccountFlowFeature.View(store: store)
			}
		}
	}
}
