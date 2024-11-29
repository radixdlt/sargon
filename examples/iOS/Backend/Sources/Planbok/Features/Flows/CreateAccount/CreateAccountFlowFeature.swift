import ComposableArchitecture
import Sargon

// MARK: - CreateAccountFlowFeature
@Reducer
public struct CreateAccountFlowFeature {
	@ObservableState
	public struct State: Equatable {
		public var nameAccount: NameNewAccountFeature.State

		public init(index: Int) {
			self.nameAccount = NameNewAccountFeature.State(index: index)
		}
	}

	public enum Action {
		public enum DelegateAction {
			case createdAccount
		}

		case nameAccount(NameNewAccountFeature.Action)
		case delegate(DelegateAction)
	}

	@Dependency(AccountsClient.self) var accountsClient

	public init() {}

	public var body: some ReducerOf<Self> {
		Scope(state: \.nameAccount, action: \.nameAccount) {
			NameNewAccountFeature()
		}

		Reduce { _, action in
			switch action {
			case let .nameAccount(.delegate(.named(name))):
				.run { send in
					try await accountsClient.createAndSaveAccount(name)
					await send(.delegate(.createdAccount))
				} catch: { error, _ in
					fatalError("TODO error handling: \(error)")
				}

			case .nameAccount:
				.none

			case .delegate:
				.none
			}
		}
	}
}

// MARK: CreateAccountFlowFeature.View
extension CreateAccountFlowFeature {
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<CreateAccountFlowFeature>
		public init(store: StoreOf<CreateAccountFlowFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			NameNewAccountFeature.View(
				store: store.scope(state: \.nameAccount, action: \.nameAccount)
			)
		}
	}
}
