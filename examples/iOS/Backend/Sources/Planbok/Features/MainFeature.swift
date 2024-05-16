import Sargon
import ComposableArchitecture

@Reducer
public struct MainFeature {
	
	@Dependency(ProfileClient.self) var profileClient
	@Dependency(AccountsClient.self) var accountsClient
	
	@Reducer(state: .equatable)
	public enum Path {
		case settings(SettingsFeature)
		case accountDetails(AccountDetailsFeature)
	}
	
	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)
		
		case deleteProfileAlert(AlertState<DeleteProfileAlert>)
		
		public enum DeleteProfileAlert {
			case confirmedDeleteProfileBDFSThenOnboard
			case confirmedEmulateFreshInstallThenTerminate
		}
	}
	
	@ObservableState
	public struct State: Equatable {
		@SharedReader(.network) var network
		@Presents var destination: Destination.State?
		public var path = StackState<Path.State>()
		public var accounts: AccountsFeature.State
		
		public init() {
			self.accounts = AccountsFeature.State()
		}
		
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		@CasePathable
		public enum ViewAction {
			case settingsButtonTapped
			case deleteWalletButtonTapped
		}
		
		@CasePathable
		public enum DelegateAction {
			case deletedWallet
			case emulateFreshInstall
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
					
				case let .element(id: _, action: action):
					switch action {
					case .accountDetails(_):
						return .none
					case .settings(_):
						return .none
					}
					
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				}
				
			case .view(.deleteWalletButtonTapped):
				state.destination = .deleteProfileAlert(.init(
					title: TextState("Delete wallet?"),
					message: TextState("Warning"),
					buttons: [
						.cancel(TextState("Cancel")),
						.destructive(
							TextState("Delete Profile & BDFS -> Onboard"),
							action: .send(.confirmedDeleteProfileBDFSThenOnboard)
						),
						.destructive(
							TextState("Emulate Fresh Install -> Restart"),
							action: .send(.confirmedEmulateFreshInstallThenTerminate)
						)
					]
				))
				return .none
				
				
			case .view(.settingsButtonTapped):
				state.path.append(.settings(SettingsFeature.State()))
				return .none
				
			case let .accounts(.delegate(.showDetailsFor(accountForDisplay))):
				state.path.append(.accountDetails(AccountDetailsFeature.State(accountForDisplay: accountForDisplay)))
				return .none
				
				
			case let .accounts(.delegate(.createNewAccount(index))):
				state.destination = .createAccount(
					CreateAccountFlowFeature.State(index: index)
				)
				return .none
				
			case .destination(.presented(.deleteProfileAlert(.confirmedEmulateFreshInstallThenTerminate))):
				log.notice("Confirmed deletion of Profile & BDFS")
				state.destination = nil
				return .run { send in
					try await profileClient.emulateFreshInstallOfAppThenRestart()
					await send(.delegate(.emulateFreshInstall))
				}
				
			case .destination(.presented(.deleteProfileAlert(.confirmedDeleteProfileBDFSThenOnboard))):
				log.notice("Confirmed deletion of Profile & BDFS (will then onboard)")
				state.destination = nil
				return .run { send in
					try await profileClient.deleteProfileAndMnemonicsThenCreateNew()
					await send(.delegate(.deletedWallet))
				}
				

			case .destination(.presented(.createAccount(.delegate(.createdAccount)))):
				state.destination = nil
				return .none
				
			default:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
		.ifLet(\.$destination, action: \.destination)
	}

}

public struct DeveloperDisclaimerBanner: View {
	public var body: some View {
		Text("Connected to a test network, not Radix main network")
			.frame(maxWidth: .infinity, alignment: .center)
			.padding(4)
			.background(Color.app.orange2)
			.font(.system(size: 12))
	}

	public init() {}
}


extension MainFeature {
	@ViewAction(for: MainFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<MainFeature>
		
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			if store.network != .mainnet {
				DeveloperDisclaimerBanner()
			}
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				VStack {
					VStack {
						Text("ProfileID:")
						Text("\(SargonOS.shared.profile.id)")
					}
					
					AccountsFeature.View(
						store: store.scope(state: \.accounts, action: \.accounts)
					)
					
					Button("Delete Wallet", role: .destructive) {
						send(.deleteWalletButtonTapped)
					}
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
			.alert($store.scope(state: \.destination?.deleteProfileAlert, action: \.destination.deleteProfileAlert))
			
		}
		
		
	}
}
