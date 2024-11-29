import ComposableArchitecture
import Foundation
import Sargon
import UniformTypeIdentifiers

// MARK: - InputDecryptionPasswordFeature
@Reducer
public struct InputDecryptionPasswordFeature {
	@CasePathable
	public enum Action: ViewAction {
		public enum DelegateAction {
			case inputtedPassword(encryptedProfile: Data, decryptionPassword: String)
		}

		@CasePathable
		public enum ViewAction {
			case passwordChanged(String)
			case confirmPasswordButtonTapped
		}

		case delegate(DelegateAction)
		case view(ViewAction)
	}

	@ObservableState
	public struct State: Equatable {
		public let encryptedProfile: Data
		public var password = ""
		public init(encryptedProfile: Data) {
			self.encryptedProfile = encryptedProfile
		}
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.passwordChanged(password)):
				state.password = password
				return .none

			case .view(.confirmPasswordButtonTapped):
				return .send(.delegate(.inputtedPassword(encryptedProfile: state.encryptedProfile, decryptionPassword: state.password)))

			case .delegate:
				return .none
			}
		}
	}
}

// MARK: InputDecryptionPasswordFeature.View
extension InputDecryptionPasswordFeature {
	@ViewAction(for: InputDecryptionPasswordFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<InputDecryptionPasswordFeature>

		public init(store: StoreOf<InputDecryptionPasswordFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				LabeledTextField(label: "Decryption password", text: $store.password.sending(\.view.passwordChanged))

				Button("Confirm") {
					send(.confirmPasswordButtonTapped)
				}
				.buttonStyle(.borderedProminent)
			}
			.padding()
		}
	}
}
