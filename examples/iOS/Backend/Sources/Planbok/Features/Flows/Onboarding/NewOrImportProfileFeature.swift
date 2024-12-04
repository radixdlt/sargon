import ComposableArchitecture
import Foundation
import Sargon
import SwiftUI

// MARK: - NewOrImportProfileFeature
@Reducer
public struct NewOrImportProfileFeature {
	public init() {}

	@ObservableState
	public struct State: Equatable {
		public init() {}
	}

	public enum Action: ViewAction {
		public enum DelegateAction {
			case createdNewEmptyProfile
			case importProfile
		}

		public enum ViewAction {
			case newProfileButtonTapped
			case importProfileButtonTapped
		}

		case delegate(DelegateAction)
		case view(ViewAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { _, action in
			switch action {
			case .view(.importProfileButtonTapped):
				.send(.delegate(.importProfile))

			case .view(.newProfileButtonTapped):
				.run { send in
					try await SargonOS.shared.newWallet(shouldPreDeriveInstances: false)
					await send(.delegate(.createdNewEmptyProfile))
				} catch: { error, _ in
					fatalError("Failed to create Profile, error: \(error)")
				}

			case .delegate:
				.none
			}
		}
	}
}

// MARK: NewOrImportProfileFeature.View
extension NewOrImportProfileFeature {
	@ViewAction(for: NewOrImportProfileFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<NewOrImportProfileFeature>

		public var body: some SwiftUI.View {
			VStack {
				Text("Existing or new user?").font(.title)

				Button("New Profile") {
					send(.newProfileButtonTapped)
				}

				Button("Import Profile") {
					send(.importProfileButtonTapped)
				}
			}
			.padding()
		}
	}
}
