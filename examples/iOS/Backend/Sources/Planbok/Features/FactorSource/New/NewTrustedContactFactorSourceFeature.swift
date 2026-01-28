import ComposableArchitecture
import Foundation
import Sargon

// MARK: - NewTrustedContactFactorSourceFeature
@Reducer
public struct NewTrustedContactFactorSourceFeature {
	@Dependency(FactorSourcesClient.self) var factorSourcesClient

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.network) var network

		public var email = ""
		public var name = ""
		public var accountAddress = ""

		public var emailAddress: EmailAddress? {
			EmailAddress(email: email)
		}

		public var displayName: DisplayName? {
			try? DisplayName(validating: name)
		}

		public var account: AccountAddress? {
			try? AccountAddress(validatingAddress: accountAddress)
		}

		public var contact: TrustedContactFactorSourceContact? {
			guard let emailAddress, let displayName else {
				return nil
			}
			return TrustedContactFactorSourceContact(
				emailAddress: emailAddress,
				name: displayName
			)
		}
	}

	public enum Action: BindableAction, ViewAction {
		public enum ViewAction {
			case addButtonTapped
			case randomEmailAddressButtonTapped
			case randomAccountAddressButtonTapped
		}

		public enum DelegateAction {
			case done
		}

		case binding(BindingAction<State>)
		case view(ViewAction)
		case delegate(DelegateAction)
	}

	public var body: some ReducerOf<Self> {
		BindingReducer()
		Reduce { state, action in
			switch action {
			case .view(.randomEmailAddressButtonTapped):
				let wordlist = bip39LanguageWordlist(language: .english)
				let word0 = wordlist.randomElement()!.word
				let word1 = wordlist.randomElement()!.word
				state.email = "\(word0)@\(word1).com"
				return .none

			case .view(.randomAccountAddressButtonTapped):
				state.accountAddress = AccountAddress.random(networkID: state.network).address
				return .none

			case .view(.addButtonTapped):
				guard let contact = state.contact,
				      let accountAddress = state.account
				else {
					return .none
				}
				let trustedContact = TrustedContactFactorSource(
					accountAddress: accountAddress,
					contact: contact
				)

				return .run { send in
					try await factorSourcesClient.addFactorSource(trustedContact.asGeneral)
					await send(.delegate(.done))
				}

			case .binding:
				return .none

			case .delegate:
				return .none
			}
		}
	}
}

extension NewTrustedContactFactorSourceFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("New Trusted Contact")
					.font(.largeTitle)

				Spacer()

				LabeledTextField(label: "Name", text: $store.name)

				LabeledTextField(label: "AccountAddress", text: $store.accountAddress) {
					buttonRandomize(with: .randomAccountAddressButtonTapped)
				}

				LabeledTextField(label: "Email", text: $store.email) {
					buttonRandomize(with: .randomEmailAddressButtonTapped)
				}

				Spacer()

				Button("Add Factor Source") {
					send(.addButtonTapped)
				}
				.buttonStyle(.borderedProminent)
				.disabled(store.contact == nil || store.account == nil)
			}
		}

		private func buttonRandomize(
			with action: HostingFeature.Action.ViewAction
		) -> some SwiftUI.View {
			Button(action: { send(action) }, label: {
				Image(systemName: "dice")
					.imageScale(.large)
					.foregroundStyle(Color.white)
					.padding()
					.background(Color.blue)
					.clipShape(.rect(cornerRadius: 10))
			})
		}
	}
}
