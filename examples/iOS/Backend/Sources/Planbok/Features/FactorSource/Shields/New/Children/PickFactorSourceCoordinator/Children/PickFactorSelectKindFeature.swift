//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture

@Reducer
public struct PickFactorSelectKindFeature {
	@ObservableState
	public struct State: Equatable {
	}
	
	@CasePathable
	public enum Action: ViewAction {
		public enum ViewAction {
			case kindButtonTapped(FactorSourceKind)
		}
        public enum DelegateAction {
            case selectedKind(FactorSourceKind)
        }
        case view(ViewAction)
        case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.kindButtonTapped(kind)):
                return .send(.delegate(.selectedKind(kind)))
            case .delegate:
                return .none
			}
		}
	}
}

extension PickFactorSelectKindFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				ScrollView {
					ForEach(FactorSourceKind.allCases) { kind in
						Button("`\(kind.title)`") {
							send(.kindButtonTapped(kind))
						}
					}
				}
                
			}
            .padding()
            .navigationTitle("Pick Factor Kind")
		}
	}
}
