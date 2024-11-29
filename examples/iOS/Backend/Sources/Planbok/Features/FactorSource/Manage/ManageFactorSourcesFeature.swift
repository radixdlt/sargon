import ComposableArchitecture
import Foundation
import Sargon
import SwiftUI

// MARK: - ManageFactorSourcesFeature
@Reducer
public struct ManageFactorSourcesFeature {
	@Dependency(FactorSourcesClient.self) var factorsClient

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.factorSources) var factorSources
	}

	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case addAllSampleValuesTapped

			case deviceButtonTapped
			case ledgerButtonTapped
			case arculusButtonTapped
			case offDeviceButtonTapped
			case securityQuestionsButtonTapped
			case trustedContactButtonTapped
		}

		case view(ViewAction)

		@CasePathable
		public enum DelegateAction {
			case navigate(Navigate)

			@CasePathable
			public enum Navigate {
				case toFactor(kind: FactorSourceKind)
			}
		}

		case delegate(DelegateAction)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce { _, action in
			switch action {
			case .view(.addAllSampleValuesTapped):
				.run { _ in
					try await factorsClient.addAllSampleFactors()
				}

			case .view(.deviceButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .device))))

			case .view(.ledgerButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .ledgerHqHardwareWallet))))

			case .view(.arculusButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .arculusCard))))

			case .view(.offDeviceButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .offDeviceMnemonic))))

			case .view(.securityQuestionsButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .securityQuestions))))

			case .view(.trustedContactButtonTapped):
				.send(.delegate(.navigate(.toFactor(kind: .trustedContact))))

			default:
				.none
			}
		}
	}
}

// MARK: ManageFactorSourcesFeature.View
extension ManageFactorSourcesFeature {
	@ViewAction(for: ManageFactorSourcesFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<ManageFactorSourcesFeature>

		public var body: some SwiftUI.View {
			VStack {
				Text("FactorSources").font(.largeTitle)
				Text("You have #\(store.state.factorSources.count) factor sources")
				Text("of #\(Set(store.state.factorSources.map(\.kind)).count) different kinds.")

				Button("ADD ALL SAMPLE FACTORS") {
					send(.addAllSampleValuesTapped)
				}

				Spacer()

				Button("Device") {
					send(.deviceButtonTapped)
				}

				Button("Ledger") {
					send(.ledgerButtonTapped)
				}

				Button("Arculus") {
					send(.arculusButtonTapped)
				}

				Button("Off Device Mnemonic") {
					send(.offDeviceButtonTapped)
				}

				Button("Security Questions") {
					send(.securityQuestionsButtonTapped)
				}

				Button("Trusted Contact") {
					send(.trustedContactButtonTapped)
				}
			}
			.padding(.bottom, 100)
		}
	}
}
