import ComposableArchitecture
import Sargon

// MARK: - NameNewAccountFeature
@Reducer
public struct NameNewAccountFeature {
	@ObservableState
	public struct State: Equatable {
		public var accountName: String
		public var errorMessage: String?
		public init(index: Int = 0) {
			self.accountName = "Unnamed \(index)"
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

extension NameNewAccountFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
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
}
