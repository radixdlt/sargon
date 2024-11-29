import ComposableArchitecture
import Sargon

// MARK: - MainFeature
@Reducer
public struct MainFeature {
	@Dependency(ProfileClient.self) var profileClient
	@Dependency(AccountsClient.self) var accountsClient

	@Reducer
	public enum Path {
		case settings(SettingsFeature)
		case manageSecurityShields(ManageSecurityShieldsFeature)
		case shieldDetails(ShieldDetailsFeature)
		case manageFactorSources(ManageFactorSourcesFeature)
		case manageSpecificFactorSources(ManageSpecificFactorSourcesFeature)
		case accountDetails(AccountDetailsFeature)
		case profileView(DebugProfileFeature)
	}

	@Reducer(state: .equatable)
	public enum Destination {
		case createAccount(CreateAccountFlowFeature)

		case newHWFactorSource(NewHWFactorSourceFeature)
		case newTrustedContact(NewTrustedContactFactorSourceFeature)
		case newSecurityQuestions(NewSecurityQuestionsFeatureCoordinator)
		case deleteProfileAlert(AlertState<DeleteProfileAlert>)

		public enum DeleteProfileAlert {
			case confirmedDeleteProfileBDFSThenOnboard
			case confirmedEmulateFreshInstallThenTerminate
		}
	}

	@ObservableState
	public struct State {
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
			case let .path(pathAction):
				switch pathAction {
				case let .element(id: _, action: action):
					switch action {
					case .settings(.delegate(.navigate(.toProfileView))):
						state.path.append(.profileView(DebugProfileFeature.State()))
						return .none

					case .settings(.delegate(.navigate(.toShields))):
						state.path.append(.manageSecurityShields(ManageSecurityShieldsFeature.State()))
						return .none

					case .settings(.delegate(.navigate(.toFactorSources))):
						state.path.append(.manageFactorSources(ManageFactorSourcesFeature.State()))
						return .none

					case let .shieldDetails(.delegate(.copyAndEdit(preset))):
						state.path.append(.manageSecurityShields(ManageSecurityShieldsFeature.State(copyAndEdit: preset)))
						return .none

					case let .manageSecurityShields(.delegate(.navigate(.toDetailsForShield(shield)))):
						state.path.append(.shieldDetails(ShieldDetailsFeature.State(shield: shield)))
						return .none

					case let .manageFactorSources(.delegate(.navigate(.toFactor(kind)))):
						state.path.append(.manageSpecificFactorSources(
							ManageSpecificFactorSourcesFeature.State(kind: kind)
						))
						return .none

					case let .manageSpecificFactorSources(.delegate(.addNew(kind))):
						if kind == .securityQuestions {
							state.destination = .newSecurityQuestions(NewSecurityQuestionsFeatureCoordinator.State())
						} else if kind == .trustedContact {
							state.destination = .newTrustedContact(NewTrustedContactFactorSourceFeature.State())
						} else {
							state.destination = .newHWFactorSource(NewHWFactorSourceFeature.State(kind: kind))
						}
						return .none

					case .profileView:
						return .none

					case .settings:
						return .none

					case .manageSecurityShields:
						return .none

					case .manageFactorSources:
						return .none

					case .manageSpecificFactorSources:
						return .none

					case .accountDetails:
						return .none

					case .shieldDetails:
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
						),
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

			case .destination(.presented(.newSecurityQuestions(.delegate(.done)))):
				state.destination = nil
				return .none

			case .destination(.presented(.newTrustedContact(.delegate(.done)))):
				state.destination = nil
				return .none

			case .destination(.presented(.newHWFactorSource(.delegate(.createdAndSavedNewFactorSource)))):
				state.destination = nil
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

// MARK: - BannerThisIsNotRadixWallet
public struct BannerThisIsNotRadixWallet: View {
	public let onMainnet: Bool
	public init(onMainnet: Bool) {
		self.onMainnet = onMainnet
	}

	public var body: some View {
		VStack(alignment: .center, spacing: 4) {
			Text("Demo app, **not** the Radix Wallet app.")
				.font(.system(size: 22))
			Text(onMainnet ? "" : "‼️ On Testnet ‼️")
				.font(.system(size: 12))
		}
		.frame(maxWidth: .infinity, alignment: .center)
		.padding(4)
		.background(Color.yellow)
		.foregroundStyle(Color.red)
		.fontWeight(.bold)
	}
}

// MARK: - MainFeature.View
extension MainFeature {
	@ViewAction(for: MainFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<MainFeature>

		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack(spacing: 0) {
				BannerThisIsNotRadixWallet(
					onMainnet: store.network == .mainnet
				)

				mainbody
			}
		}

		var mainbody: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				VStack {
					if let profile = try? SargonOS.shared.profile() {
						VStack {
							Text("ProfileID:")
							Text("\(profile.id)")
						}
					} else {
						Text("NO PROFILE")
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
				destinationView(store: store)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.createAccount,
					action: \.destination.createAccount
				)
			) { store in
				CreateAccountFlowFeature.View(store: store)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.newHWFactorSource,
					action: \.destination.newHWFactorSource
				)
			) { store in
				NewHWFactorSourceFeature.View(store: store)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.newTrustedContact,
					action: \.destination.newTrustedContact
				)
			) { store in
				NewTrustedContactFactorSourceFeature.View(store: store)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.newSecurityQuestions,
					action: \.destination.newSecurityQuestions
				)
			) { store in
				NewSecurityQuestionsFeatureCoordinator.View(store: store)
			}
			.alert($store.scope(state: \.destination?.deleteProfileAlert, action: \.destination.deleteProfileAlert))
		}

		@ViewBuilder
		func destinationView(store: StoreOf<MainFeature.Path>) -> some SwiftUI.View {
			switch store.case {
			case let .settings(store):
				SettingsFeature.View(store: store)

			case let .manageSecurityShields(store):
				ManageSecurityShieldsFeature.View(store: store)

			case let .manageFactorSources(store):
				ManageFactorSourcesFeature.View(store: store)

			case let .manageSpecificFactorSources(store):
				ManageSpecificFactorSourcesFeature.View(store: store)

			case let .accountDetails(store):
				AccountDetailsFeature.View(store: store)

			case let .shieldDetails(store):
				ShieldDetailsFeature.View(store: store)

			case let .profileView(store):
				DebugProfileFeature.View(store: store)
			}
		}
	}
}
