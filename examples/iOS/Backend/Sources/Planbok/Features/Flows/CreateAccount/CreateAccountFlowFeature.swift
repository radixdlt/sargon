import Sargon
import ComposableArchitecture

@Reducer
public struct CreateAccountFlowFeature {

	@ObservableState
	public struct State: Equatable {
		public var nameAccount: NameNewAccountFeature.State
		
		public init() {
			self.nameAccount = NameNewAccountFeature.State()
		}
	}
	
	public enum Action {
		public enum DelegateAction {
			case createdAccount
		}
		case nameAccount(NameNewAccountFeature.Action)
		case delegate(DelegateAction)
	}
	
	@Dependency(AccountsClient.self) var accountsClient
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.nameAccount, action: \.nameAccount) {
			NameNewAccountFeature()
		}
		
		Reduce { state, action in
			switch action {

			case let .nameAccount(.delegate(.named(name))):
				return .run { send in
					try await accountsClient.createAndSaveAccount(.mainnet, name)
					await send(.delegate(.createdAccount))
				} catch: { _, error in
					fatalError("TODO error handling: \(error)")
				}

			case .nameAccount(_):
				return .none
				
			case .delegate:
				return .none
			}
		}
	}
}

extension CreateAccountFlowFeature {
	public struct View: SwiftUI.View {

		@Bindable var store: StoreOf<CreateAccountFlowFeature>
		public init(store: StoreOf<CreateAccountFlowFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			NameNewAccountFeature.View(
				store: store.scope(state: \.nameAccount, action: \.nameAccount)
			)
		}
	}

}
