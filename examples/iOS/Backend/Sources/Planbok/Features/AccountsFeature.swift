import Sargon
import ComposableArchitecture


@Reducer
public struct AccountsFeature {
	@Dependency(AccountsClient.self) var accountsClient
	
	public init() {}
	
	@ObservableState
	public struct State: Equatable {
		@SharedReader(.accounts) var accounts
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case accountCardTapped(Account)
			case createNewAccountButtonTapped
		}
		public enum DelegateAction {
			case createNewAccount(index: Int)
			case showDetailsFor(Account)
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {

			case .view(.createNewAccountButtonTapped):
				return .send(.delegate(.createNewAccount(index: state.accounts.count)))
				
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
				
				ScrollView {
					ForEach(store.state.accounts) { account in
						VStack {
							AccountCardView(account: account) {
								send(.accountCardTapped(account))
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
