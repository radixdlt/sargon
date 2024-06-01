//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct FactorSourcesFeature {

    @ObservableState
    public struct State: Equatable {
		@SharedReader(.factorSources) var factorSources
    }
    
    @CasePathable
    public enum Action: ViewAction {
        
        @CasePathable
        public enum ViewAction {
            case deviceFactorSourcesButtonTapped
        }
        
        case view(ViewAction)
        
        @CasePathable
        public enum DelegateAction {
            case navigate(Navigate)
            
            @CasePathable
            public enum Navigate {
                case toDeviceFactorSources
            }
        }
        
        case delegate(DelegateAction)
    }
    
    public init() {}
    
    public var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
            case .view(.deviceFactorSourcesButtonTapped):
                return .send(.delegate(.navigate(.toDeviceFactorSources)))
                
        
            default:
                return .none
                
            }
        }
    }
}

extension FactorSourcesFeature {
    
    @ViewAction(for: FactorSourcesFeature.self)
    public struct View: SwiftUI.View {
        
        @Bindable public var store: StoreOf<FactorSourcesFeature>
        
        public var body: some SwiftUI.View {
            VStack {
                Text("FactorSources").font(.largeTitle)
        
				Spacer()
				
				Button("Device Factor Sources") {
					send(.deviceFactorSourcesButtonTapped)
				}
				
           
                
            }
            .padding(.bottom, 100)
        }
    }
    
}

