import Sargon
import ComposableArchitecture

@Reducer
public struct MainFeature {
	
	@Dependency(ProfileClient.self) var profileClient

	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
		
		case settings(SettingsFeature)
		
		case alert(AlertState<Alert>)
		
		public enum Alert {
			case confirmedDeleteWallet
		}
	}
	
	@ObservableState
	public struct State: Equatable {
		
		@Presents var destination: Destination.State?
		
		public var accounts: AccountsFeature.State
	
		public init() {
			self.accounts = AccountsFeature.State(accounts: SargonOS.shared.accounts())
		}
		
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case settingsButtonTapped
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
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.accounts, action: \.accounts) {
			AccountsFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .view(.settingsButtonTapped):
				state.destination = .settings(SettingsFeature.State())
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
					CreateAccountFlowFeature.State()
				)
				return .none
				
			case .destination(.presented(.alert(.confirmedDeleteWallet))):
				log.notice("Confirmed deletion of wallet")
				state.destination = nil
				return .run { send in
					try await profileClient.deleteProfileAndMnemonicsThenCreateNew()
					await send(.delegate(.deletedWallet))
				}
			
			case .destination(.presented(.createAccount(.delegate(.createdAccount)))):
				state.destination = nil
				state.accounts.accounts = SargonOS.shared.accounts()
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
			NavigationStack {
				VStack {
					VStack {
						Text("ProfileID:")
						Text("\(SargonOS.shared.profile.id)")
					}
					
					AccountsFeature.View(
						store: store.scope(state: \.accounts, action: \.accounts)
					)
				}
				.sheet(
					item: $store.scope(
						state: \.destination?.settings,
						action: \.destination.settings
					)
				) { store in
					NavigationView {
						SettingsFeature.View(store: store)
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
				.alert($store.scope(state: \.destination?.alert, action: \.destination.alert))
				.toolbar {
					ToolbarItem(placement: .primaryAction) {
						Button("Settings") {
							send(.settingsButtonTapped)
						}
					}
				}
			}
		}
	}
}
