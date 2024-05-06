import Sargon
import ComposableArchitecture

@Reducer
public struct MainFeature {
	
	@Dependency(ProfileClient.self) var profileClient
	
	@Reducer(state: .equatable)
	public enum Path {
		case settings(SettingsFeature)
		case accountDetails(AccountDetailsFeature)
	}
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
		
		case alert(AlertState<Alert>)
		
		public enum Alert {
			case confirmedDeleteWallet
		}
	}
	
	@ObservableState
	public struct State: Equatable {
		
		@Presents var destination: Destination.State?
		public var path = StackState<Path.State>()
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
		case path(StackAction<Path.State, Path.Action>)
	
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
				
			case .path(let pathAction):
				switch pathAction {

				case .element(id: let id, action: let action):
					switch action {
						
					}
		
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				
				}
				
			case .view(.settingsButtonTapped):
				state.path.append(.settings(SettingsFeature.State()))
				return .none

			case let .accounts(.delegate(.showDetailsFor(account))):
				state.path.append(.accountDetails(AccountDetailsFeature.State(account: account)))
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
		.forEach(\.path, action: \.path)
		.ifLet(\.$destination, action: \.destination)
	}
}

extension MainFeature {
	@ViewAction(for: MainFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<MainFeature>
		
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				VStack {
					VStack {
						Text("ProfileID:")
						Text("\(SargonOS.shared.profile.id)")
					}
					AccountsFeature.View(
						store: store.scope(state: \.accounts, action: \.accounts)
					)
				}
				.toolbar {
					ToolbarItem(placement: .primaryAction) {
						Button("Settings") {
							send(.settingsButtonTapped)
						}
					}
				}
			} destination: { store in
				switch store.case {
				case let .settings(store):
					SettingsFeature.View(store: store)
				case let .accountDetails(store):
					AccountDetailsFeature.View(store: store)
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
			
		}
		
	
	}
}
