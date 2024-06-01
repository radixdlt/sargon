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
public struct ManageFactorSourcesFeature {

    @ObservableState
    public struct State: Equatable {
		@SharedReader(.factorSources) var factorSources
    }
    
    @CasePathable
    public enum Action: ViewAction {
        
        @CasePathable
        public enum ViewAction {
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
        Reduce { state, action in
            switch action {
            case .view(.deviceButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .device))))
			
			case .view(.ledgerButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .ledgerHqHardwareWallet))))
		  
			case .view(.arculusButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .arculusCard))))
		  
			case .view(.offDeviceButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .offDeviceMnemonic))))
		  
			case .view(.securityQuestionsButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .securityQuestions))))
		  
			case .view(.trustedContactButtonTapped):
				return .send(.delegate(.navigate(.toFactor(kind: .trustedContact))))
        
            default:
                return .none
                
            }
        }
    }
}

extension ManageFactorSourcesFeature {
    
    @ViewAction(for: ManageFactorSourcesFeature.self)
    public struct View: SwiftUI.View {
        
        @Bindable public var store: StoreOf<ManageFactorSourcesFeature>
        
        public var body: some SwiftUI.View {
            VStack {
                Text("FactorSources").font(.largeTitle)
        
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
				
				Button("Security Questions") {
					send(.securityQuestionsButtonTapped)
				}
				
				Button("Trusted Contact") {
					send(.trustedContactButtonTapped)
				}
				
				Button("Off Device Mnemonic") {
					send(.offDeviceButtonTapped)
				}
           
                
            }
            .padding(.bottom, 100)
        }
    }
    
}

