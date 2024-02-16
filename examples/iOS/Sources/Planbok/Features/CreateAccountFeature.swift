@Reducer
public struct CreateAccountFeature {
	
	@ObservableState
	public struct State: Equatable {
		public let walletHolder: WalletHolder
		public var accountName = ""
		public init(walletHolder: WalletHolder) {
			self.walletHolder = walletHolder
		}
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action {
		case accountNameChanged(String)
		case createAccountButtonTapped
		case createdAccount
	}
	
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<CreateAccountFeature>
		public init(store: StoreOf<CreateAccountFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Create Account").font(.largeTitle)
				Spacer()
				LabeledTextField(label: "Account Name", text: $store.accountName.sending(\.accountNameChanged))
				Spacer()
				Button("Create Account") {
					store.send(.createAccountButtonTapped)
				}
			}
			.padding()
		}
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .accountNameChanged(name):
				state.accountName = name
				return .none
				
			case .createAccountButtonTapped:
				guard let displayName = try? DisplayName(validating: state.accountName) else {
					return .none
				}
				do {
					_ = try state.walletHolder.wallet.createAndSaveNewAccount(
						networkId: .mainnet,
						name: displayName
					)
					return .send(.createdAccount)
				} catch {
					fatalError("Handle error: \(error)")
				}
			case .createdAccount:
				return .none
			}
		}
	}
}

public struct LabeledTextField: SwiftUI.View {
	public let label: LocalizedStringKey
	@Binding public var text: String
	public var body: some View {
		VStack {
			Text(label)
			TextField(label, text: $text)
		}
	}
}
