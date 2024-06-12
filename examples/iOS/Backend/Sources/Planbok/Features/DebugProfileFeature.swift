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
			case copyNode(String)
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
				
			case let .view(.copyNode(nodeValue)):
				// this is NOT JSON...
				pasteboardClient.copyString(nodeValue)
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
			Group {
				if let profileJSONString = store.profileJSONString {
					if let jsonNode = profileJSONString.jsonNode(sortingStrategy: .none) {
						JSONViewer(
							rootNode: jsonNode,
							fontConfiguration: .constant(.init(
								keyFont: .system(size: 12),
								valueFont: .system(size: 10)
							)),
							initialNodeExpandStategy: .all
						) { event in
							switch event {
							case let .onDoubleTap(node):
								send(.copyNode(node.value))
							}
						}
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
