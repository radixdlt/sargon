@Reducer
public struct OnboardingFeature {
	
	
	@Reducer(state: .equatable)
	public enum Path {
		case createAccount(CreateAccountFeature)
	}
	
	@ObservableState
	public struct State: Equatable {
		public var path = StackState<Path.State>()
		public var createAccount: CreateAccountFeature.State
		
		public init(walletHolder: WalletHolder) {
			self.createAccount = CreateAccountFeature.State(walletHolder: walletHolder)
		}
		
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action {
		case path(StackAction<Path.State, Path.Action>)
		case createAccount(CreateAccountFeature.Action)
		
		case createdAccount(with: WalletHolder)
	}
	
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<OnboardingFeature>
		public init(store: StoreOf<OnboardingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				CreateAccountFeature.View(
					store: store.scope(state: \.createAccount, action: \.createAccount)
				)
			} destination: { _ in
				Text("Never seen")
			}
		}
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.createAccount, action: \.createAccount) {
			CreateAccountFeature()
		}
		Reduce { state, action in
			switch action {

			case .createAccount(.createdAccount):
				return .send(.createdAccount(with: state.createAccount.walletHolder))
			default:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
	}
}
