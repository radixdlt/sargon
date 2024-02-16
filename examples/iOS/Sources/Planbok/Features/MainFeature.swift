@Reducer
public struct MainFeature {
	
	@Dependency(\.keychain) var keychain
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.accounts, action: \.accounts) {
			AccountsFeature()
		}
		Reduce { state, action in
			switch action {
			
			case .accounts(.delegate(.deleteWallet)):
				state.destination = .alert(.init(
					title: TextState("Delete wallet?"),
					message: TextState("Warning"),
					buttons: [
						.cancel(TextState("Cancel")),
						.destructive(
							TextState("Delete Wallet and mnemonic"),
							action: .send(.confirmedDeleteWallet)
						)
					]
				))
				return .none
				
			case .accounts(.delegate(.createNewAccount)):
				state.destination = .createAccount(
					CreateAccountFeature.State(
						walletHolder: state.walletHolder
					)
				)
				return .none
				
			case .destination(.presented(.alert(.confirmedDeleteWallet))):
				print("⚠️ Confirmed deletion of wallet")
				state.destination = nil
				let profileID = state.walletHolder.wallet.profile().id
				do {
					try keychain.deleteDataForKey(SecureStorageKey.profileSnapshot(profileId: profileID))
					try keychain.deleteDataForKey(SecureStorageKey.activeProfileId)
					return .send(.delegate(.deletedWallet))
				} catch {
					fatalError("Fix error handling, error: \(error)")
				}
			
			case .destination(.presented(.createAccount(.createdAccount))):
				state.destination = nil
				state.accounts.refresh() // FIXME: we really do not want this.
				return .none
			
			default:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFeature)
		case alert(AlertState<Alert>)
		
		public enum Alert {
			case confirmedDeleteWallet
		}
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
		public enum DelegateAction {
			case deletedWallet
		}
		case destination(PresentationAction<Destination.Action>)
		case accounts(AccountsFeature.Action)
		
		case delegate(DelegateAction)
		
	}
	
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<MainFeature>
		
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				VStack {
					Text("ProfileID:")
					Text("\(store.state.walletHolder.wallet.profile().id)")
				}
				
				AccountsFeature.View(
					store: store.scope(state: \.accounts, action: \.accounts)
				)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.createAccount,
					action: \.destination.createAccount
				)
			) { store in
				CreateAccountFeature.View(store: store)
			}
			.alert($store.scope(state: \.destination?.alert, action: \.destination.alert))
		}
	}
}
