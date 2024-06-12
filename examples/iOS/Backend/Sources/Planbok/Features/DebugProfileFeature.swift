//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-12.
//

import Foundation
import Sargon
import ComposableArchitecture
import JSONViewer
import AsyncExtensions


@Reducer
public struct DebugProfileFeature {
	
	@Dependency(ProfileClient.self) var profileClient
	@Dependency(PasteboardClient.self) var pasteboardClient
	
	@ObservableState
	public struct State {
		public var profileJSONString: String?
	}
	
	public enum Action: ViewAction, Sendable {
		public enum ViewAction: Sendable {
			case appear
			case copyButtonTapped
		}
		public enum InternalAction: Sendable {
			case loadedProfileString(String)
		}
		case view(ViewAction)
		case `internal`(InternalAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.appear):
				return .run { send in
					let jsonString = profileClient.activeProfile().toJSONString(prettyPrinted: true)
					await send(.internal(.loadedProfileString(jsonString)))
				}

			case .view(.copyButtonTapped):
				guard let profileJSONString = state.profileJSONString else {
					return .none
				}
				pasteboardClient.copyString(profileJSONString)
				return .none
				
			case let .internal(.loadedProfileString(profileJSONString)):
				state.profileJSONString = profileJSONString
				return .none
			}
		}
	}
}

extension DebugProfileFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
				ScrollView {
					if let profileJSONString = store.profileJSONString {
						if let jsonNode = profileJSONString.jsonNode(sortingStrategy: .none) {
							JSONViewer(rootNode: jsonNode)
						} else {
							Text("Failed to create 'JSONNode' from JSON String.")
						}
					} else {
						Text("Loading")
					}
				}
				.onAppear {
					send(.appear)
				}
				.toolbar {
					ToolbarItem(placement: .topBarTrailing) {
						Button("Copy") {
							send(.copyButtonTapped)
						}
						.foregroundStyle(.blue)
						.buttonStyle(.plain)
					}
				}
		}
	}
}



//// MARK: - JSONView
//public struct JSONView: SwiftUI.View {
//	let jsonString: String
//	public init(jsonString: String) {
//		self.jsonString = jsonString
//	}
//
//	public var body: some View {
//		UIKitJSONView(jsonString: jsonString)
//			.padding(.leading, -60) // we hide the "line number" view on the left which eats up precious widdth,zoo
//	}
//}
//
//// MARK: - UIKitJSONView
//@MainActor
//struct UIKitJSONView: UIViewRepresentable {
//	let jsonPreview: JSONPreview
//	init(jsonString: String) {
//		let jsonPreview = JSONPreview()
//		jsonPreview.preview(jsonString)
//		self.jsonPreview = jsonPreview
//	}
//
//	func makeUIView(context: Context) -> JSONPreview {
//		jsonPreview
//	}
//
//	func updateUIView(_ uiView: UIViewType, context: Context) {}
//}
