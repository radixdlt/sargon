import ComposableArchitecture
import Foundation
import Sargon
import SwiftUI

// MARK: - EditLabelOffDeviceMnemonicFactor
@Reducer
public struct EditLabelOffDeviceMnemonicFactor {
	@Dependency(FactorSourcesClient.self) var factorSourcesClient
	@Dependency(\.dismiss) var dismiss

	@ObservableState
	public struct State: Equatable {
		public let factorSource: OffDeviceMnemonicFactorSource
		public var label: String
		public var displayName: DisplayName? {
			try? DisplayName(validating: label)
		}

		public init(factorSource: OffDeviceMnemonicFactorSource) {
			self.factorSource = factorSource
			self.label = factorSource.hint.displayName.value
		}
	}

	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case labelChanged(String)
			case confirmButtonTapped
		}

		case view(ViewAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.labelChanged(label)):
				state.label = label
				return .none
			case .view(.confirmButtonTapped):
				guard let displayName = state.displayName else {
					return .none
				}

				return .run { [factorSource = state.factorSource] _ in
					var factorSource = factorSource
					factorSource.hint.displayName = displayName
					try await factorSourcesClient.updateFactorSource(factorSource.asGeneral)
					await dismiss()
				}
			}
		}
	}
}

extension EditLabelOffDeviceMnemonicFactor {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				LabeledTextField(label: "", text: $store.label.sending(\.view.labelChanged))
				Button("Confirm") {
					send(.confirmButtonTapped)
				}
				.buttonStyle(.borderedProminent)
				.disabled(store.displayName == nil)
			}
		}
	}
}

// MARK: - ManageSpecificFactorSourcesFeature
@Reducer
public struct ManageSpecificFactorSourcesFeature {
	@Reducer(state: .equatable)
	public enum Destination {
		case decryptSecurityQuestions(DecryptSecurityQuestionsFeatureCoordinator)
		case editLabelOffDeviceMnemonicFactor(EditLabelOffDeviceMnemonicFactor)
	}

	@ObservableState
	public struct State {
		@SharedReader(.factorSources) var factorSources

		@Presents var destination: Destination.State?
		public let kind: FactorSourceKind
	}

	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case addNewButtonTapped
			case factorSourceActionButtonTapped(FactorSource)
		}

		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)

		@CasePathable
		public enum DelegateAction {
			case addNew(FactorSourceKind)
		}

		case delegate(DelegateAction)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addNewButtonTapped):
				return .send(.delegate(.addNew(state.kind)))

			case let .view(.factorSourceActionButtonTapped(factorSource)):
				if let securityQuestions = factorSource.asSecurityQuestions {
					state.destination = .decryptSecurityQuestions(
						DecryptSecurityQuestionsFeatureCoordinator.State(
							securityQuestionsFactorSource: securityQuestions
						)
					)
				} else if let offDeviceMnemonic = factorSource.asOffDeviceMnemonic {
					state.destination = .editLabelOffDeviceMnemonicFactor(EditLabelOffDeviceMnemonicFactor.State(factorSource: offDeviceMnemonic))
				} else {
					log.warning("FactorSource tapped but no action performed: \(factorSource)")
				}
				return .none

			case .destination(.presented(.decryptSecurityQuestions(.delegate(.done)))):
				state.destination = nil
				return .none

			default:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}
}

extension ManageSpecificFactorSourcesFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>

		public var kind: FactorSourceKind {
			store.kind
		}

		public var factors: IdentifiedArrayOf<FactorSource> {
			store.factorSources.filter(kind: kind)
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("\(kind) Factors").font(.largeTitle)

				if factors.isEmpty {
					Text("You have no factors")
				} else {
					ScrollView {
						ForEach(factors, id: \.id) { factorSource in
							VStack {
								FactorSourceCardView(factorSource: factorSource) {
									send(.factorSourceActionButtonTapped(factorSource))
								}
							}
						}
					}
				}

				Spacer()

				Button("Add New") {
					send(.addNewButtonTapped)
				}
			}
			.padding(.bottom, 100)
			.sheet(
				item: $store.scope(
					state: \.destination?.decryptSecurityQuestions,
					action: \.destination.decryptSecurityQuestions
				)
			) { store in
				DecryptSecurityQuestionsFeatureCoordinator.View(store: store)
			}
			.sheet(
				item: $store.scope(
					state: \.destination?.editLabelOffDeviceMnemonicFactor,
					action: \.destination.editLabelOffDeviceMnemonicFactor
				)
			) { store in
				EditLabelOffDeviceMnemonicFactor.View(store: store)
			}
		}
	}
}
