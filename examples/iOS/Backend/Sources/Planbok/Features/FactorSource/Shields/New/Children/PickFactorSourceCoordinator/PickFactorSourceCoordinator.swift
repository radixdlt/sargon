//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct PickFactorSourceCoordinator {
	
    @Reducer(state: .equatable)
    public enum Path {
        case pickFactorSource(PickFactorSourceFeature)
    }
    
    @ObservableState
    public struct State: Equatable {
        public var path = StackState<Path.State>()
        public var pickKind: PickFactorSelectKindFeature.State
        
//        @Presents var destination: Destination.State?
        
        public init() {
            self.pickKind = PickFactorSelectKindFeature.State()
        }
    }
    
    @CasePathable
    public enum Action {
        @CasePathable
        public enum DelegateAction {
            case done
        }
        
//        case destination(PresentationAction<Destination.Action>)
        case path(StackAction<Path.State, Path.Action>)
        case pickKind(PickFactorSelectKindFeature.Action)
        case delegate(DelegateAction)
    }
    
    public var body: some ReducerOf<Self> {
        Scope(state: \.pickKind, action: \.pickKind) {
            PickFactorSelectKindFeature()
        }
        Reduce { state, action in
            switch action {

            case let .pickKind(.delegate(.selectedKind(kind))):
                state.path.append(.pickFactorSource(PickFactorSourceFeature.State(kind: kind)))
                return .none
                
            case .path(.element(id: _, action: .pickFactorSource(.delegate(.done)))):
                log.fault("PickFactorSourceCoordinator delegate done")
                return .send(.delegate(.done))
                
            case .pickKind:
                return .none

            case .path:
                return .none

            case .delegate:
                return .none
            }
        }
        .forEach(\.path, action: \.path)
//        .ifLet(\.$destination, action: \.destination)
    }
}

extension PickFactorSourceCoordinator {
	public typealias HostingFeature = Self
	
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		
        public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		
        
        public var body: some SwiftUI.View {
            NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
                PickFactorSelectKindFeature.View(
                    store: store.scope(state: \.pickKind, action: \.pickKind)
                )
            } destination: { store in
                switch store.case {
                case let .pickFactorSource(store):
                    PickFactorSourceFeature.View(store: store)
                }
            }
//            .sheet(
//                item: $store.scope(state: \.destination?.foo, action: \.destination.foo)
//            ) { store in
//                Foo.View(store: store)
//            }

        }
        
	}
}
