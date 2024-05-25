import Sargon
import ComposableArchitecture


@Reducer
public struct AccountsFeature {
	
	public init() {}
	
	@ObservableState
	public struct State: Equatable {
		@SharedReader(.network) var network
		@SharedReader(.accountsForDisplay) var accountsForDisplay
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case accountCardTapped(AccountForDisplay)
			case createNewAccountButtonTapped
		}
		public enum DelegateAction {
			case createNewAccount(index: Int)
			case showDetailsFor(AccountForDisplay)
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {

			case .view(.createNewAccountButtonTapped):
				return .send(.delegate(.createNewAccount(index: state.accountsForDisplay.count)))
				
			case let .view(.accountCardTapped(account)):
				return .send(.delegate(.showDetailsFor(account)))
				
			default: return .none
			}
		}
	}
}

extension AccountsFeature {
	@ViewAction(for: AccountsFeature.self)
	public struct View: SwiftUI.View {
		
		public let store: StoreOf<AccountsFeature>
		
		public init(store: StoreOf<AccountsFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Accounts").font(.largeTitle)
				
				if store.state.accountsForDisplay.isEmpty {
					Text("You dont have any accounts on \(store.state.network.description)")
				} else {
					ScrollView {
						ForEach(store.state.accountsForDisplay) { accountForDisplay in
							VStack {
								AccountCardView(accountForDisplay: accountForDisplay) {
									send(.accountCardTapped(accountForDisplay))
								}
							}
						}
					}
				}
				
				Spacer()
				
				Button("Create New Account") {
					send(.createNewAccountButtonTapped)
				}
				
			}
			.padding()
		}
	}
}
