import Sargon
import ComposableArchitecture

@Reducer
public struct CreateAccountFlowFeature {
	
	@Reducer(state: .equatable)
	public enum Path {
		case selectGradient(SelectGradientFeature)
	}
	
	@ObservableState
	public struct State: Equatable {
		public var path = StackState<Path.State>()
		public var nameAccount: NameNewAccountFeature.State
		
		public init() {
			self.nameAccount = NameNewAccountFeature.State()
		}
		
	}
	
	public enum Action {
		public enum DelegateAction {
			case createdAccount
		}
		case path(StackAction<Path.State, Path.Action>)
		case nameAccount(NameNewAccountFeature.Action)
		case delegate(DelegateAction)
	}
	
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<CreateAccountFlowFeature>
		public init(store: StoreOf<CreateAccountFlowFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				NameNewAccountFeature.View(
					store: store.scope(state: \.nameAccount, action: \.nameAccount)
				)
			} destination: { store in
				switch store.state {
				case .selectGradient:
					if let store = store.scope(state: \.selectGradient, action: \.selectGradient) {
						SelectGradientFeature.View(store: store)
					}
				}
			}
		}
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.nameAccount, action: \.nameAccount) {
			NameNewAccountFeature()
		}
		
		Reduce { state, action in
			switch action {

			case let .nameAccount(.delegate(.named(name))):
				state.path.append(.selectGradient(.init(name: name)))
				return .none

			case .path(let pathAction):
				switch pathAction {
					
				case let .element(
					id: _,
					action: .selectGradient(.delegate(.selected(_, displayName)))
				):
					
					return .run { send in
						try await SargonOS.shared.createAccount(named: displayName)
						await send(.delegate(.createdAccount))
					} catch: { _, error in
						fatalError("TODO error handling: \(error)")
					}
						
				case .element(id: _, action: _):
					return .none
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				}

			case .nameAccount(.view):
				return .none
				
			case .delegate:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
	}
}
