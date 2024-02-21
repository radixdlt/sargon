@Reducer
public struct MainFeature {
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
		case previewAddresses(PreviewAddressesFeature)
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
	
	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case previewAddressButtonTapped
		}
		@CasePathable
		public enum DelegateAction {
			case deletedWallet
		}
		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)
		case accounts(AccountsFeature.Action)
		
		case delegate(DelegateAction)
		
	}
	
	@Dependency(\.keychain) var keychain
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.accounts, action: \.accounts) {
			AccountsFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .view(.previewAddressButtonTapped):
				state.destination = .previewAddresses(PreviewAddressesFeature.State())
				return .none
			
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
					CreateAccountFlowFeature.State(
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
			
			case .destination(.presented(.createAccount(.delegate(.createdAccount)))):
				state.destination = nil
				state.accounts.refresh() // FIXME: we really do not want this.
				return .none
			
			default:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
	
	@ViewAction(for: MainFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<MainFeature>
		
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			NavigationView {
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
						state: \.destination?.previewAddresses,
						action: \.destination.previewAddresses
					)
				) { store in
					PreviewAddressesFeature.View(store: store)
				}
				.sheet(
					item: $store.scope(
						state: \.destination?.createAccount,
						action: \.destination.createAccount
					)
				) { store in
					CreateAccountFlowFeature.View(store: store)
				}
				.alert($store.scope(state: \.destination?.alert, action: \.destination.alert))
				.toolbar {
					ToolbarItem(placement: .topBarTrailing) {
						Button("Preview") {
							send(.previewAddressButtonTapped)
						}
					}
				}
			}
		}
	}
}
