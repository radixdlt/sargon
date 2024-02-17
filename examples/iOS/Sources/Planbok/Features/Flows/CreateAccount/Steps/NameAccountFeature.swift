@Reducer
public struct NameNewAccountFeature {
	
	@ObservableState
	public struct State: Equatable {
		public let walletHolder: WalletHolder
		public var accountName = ""
		public var errorMessage: String?
		public init(walletHolder: WalletHolder) {
			self.walletHolder = walletHolder
		}
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action: ViewAction {
		public enum Delegate {
			case named(DisplayName)
		}
		@CasePathable
		public enum ViewAction {
			case accountNameChanged(String)
			case continueButtonTapped
		}
		case delegate(Delegate)
		case view(ViewAction)
	}
	
	@ViewAction(for: NameNewAccountFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<NameNewAccountFeature>
		public init(store: StoreOf<NameNewAccountFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Name Account").font(.largeTitle)
				Spacer()
				LabeledTextField(label: "Account Name", text: $store.accountName.sending(\.view.accountNameChanged))
				if let error = store.state.errorMessage {
					Text("\(error)")
						.foregroundStyle(Color.red)
						.font(.footnote)
						.fontWeight(.bold)
				}
				Spacer()
				Button("Continue") {
					send(.continueButtonTapped)
				}
				.buttonStyle(.borderedProminent)
			}
			.padding()
		}
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.accountNameChanged(name)):
				state.errorMessage = nil
				state.accountName = name
				return .none
				
			case .view(.continueButtonTapped):
				state.errorMessage = nil
				do {
					let displayName = try DisplayName(validating: state.accountName)
					return .send(.delegate(.named(displayName)))
				} catch {
					state.errorMessage = "Invalid DisplayName, can't be empty or too long."
					return .none
				}
				
			case .delegate:
				return .none
		
			}
		}
	}
}
