import Sargon
import ComposableArchitecture

@Reducer
public struct AccountsFeature {
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			
			case .view(.createNewAccountButtonTapped):
				return .send(.delegate(.createNewAccount))
			
			case .view(.deleteWalletButtonTapped):
				return .send(.delegate(.deleteWallet))
			
			default: return .none
			}
		}
	}

	@ObservableState
	public struct State: Equatable {
		public var walletHolder: WalletHolder
		
		// FIXME: We really do not want this
		mutating func refresh() {
			walletHolder.refresh()
		}
		
		public var profile: Profile {
			walletHolder.wallet.profile()
		}
		
		public var currentNetworkID: NetworkId {
			profile.appPreferences.gateways.current.network.id
		}
		
		public var network: ProfileNetwork {
			profile.networks.first(where: { $0.id == currentNetworkID })!
		}
		
		public var accounts: Accounts {
			network.accounts
		}
		
		public init(walletHolder: WalletHolder) {
			self.walletHolder = walletHolder
		}
		
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case createNewAccountButtonTapped
			case deleteWalletButtonTapped
		}
		public enum DelegateAction {
			case createNewAccount
			case deleteWallet
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	@ViewAction(for: AccountsFeature.self)
	public struct View: SwiftUI.View {
		
		public let store: StoreOf<AccountsFeature>
		
		public init(store: StoreOf<AccountsFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Accounts").font(.largeTitle)
				
				ForEach(store.state.accounts) { account in
					VStack {
						AccountView(account: account)
					}
				}
				
				Spacer()
				
				Button("Create New Account") {
					send(.createNewAccountButtonTapped)
				}
				
				Button("Delete Wallet", role: .destructive) {
					send(.deleteWalletButtonTapped)
				}
			}
			.padding()
		}
	}
}
