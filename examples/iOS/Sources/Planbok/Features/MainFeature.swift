@Reducer
public struct MainFeature {
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.accounts, action: \.accounts) {
			AccountsFeature()
		}
		Reduce { state, action in
			switch action {
			case .accounts(.delegate(.createNewAccount)):
				state.destination = .createAccount(CreateAccountFeature.State(walletHolder: state.walletHolder))
				return .none
			case .destination(.presented(.createAccount(.createdAccount))):
				state.destination = nil
				state.accounts.refresh()
				return .none
			default:
				print("MainFeature ignored action: \(action)")
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFeature)
	}
	
	@ObservableState
	public struct State: Equatable {
		
		@Presents var destination: Destination.State?
		
		public var accounts: AccountsFeature.State
		public let walletHolder: WalletHolder
	
		public init(walletHolder: WalletHolder) {
			self.walletHolder = walletHolder
			self.accounts = AccountsFeature.State(walletHolder: walletHolder)
		}
		
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action {
		case destination(PresentationAction<Destination.Action>)
		case accounts(AccountsFeature.Action)
	}
	
	public struct View: SwiftUI.View {
		
//		public let store: StoreOf<MainFeature>
		@Bindable public var store: StoreOf<MainFeature>
		
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
//			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
//				SyncUpsListView(
//					store: store.scope(state: \.syncUpsList, action: \.syncUpsList)
//				)
//			} destination: { store in
//				switch store.case {
//				case let .detail(store):
//					SyncUpDetailView(store: store)
//				case let .meeting(meeting, syncUp):
//					MeetingView(meeting: meeting, syncUp: syncUp)
//				case let .record(store):
//					RecordMeetingView(store: store)
//				}
//			}
			AccountsFeature.View(store: store.scope(state: \.accounts, action: \.accounts))
				.sheet(item: $store.scope(state: \.destination?.createAccount, action: \.destination.createAccount)) { store in
					CreateAccountFeature.View(store: store)
				}
		}
	}
}
