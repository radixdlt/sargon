//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

@Reducer
public struct WelcomeFeature {
	public init() {}
	
	@ObservableState
	public struct State: Equatable {
		public init() {}
	}
	
	public enum Action: ViewAction {
		public enum DelegateAction {
			case done
		}
		public enum ViewAction {
			case continueButtonTapped
		}
		case delegate(DelegateAction)
		case view(ViewAction)
	}
	
	@ViewAction(for: WelcomeFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<WelcomeFeature>
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Welcome to Sargon demo").font(.title)
				ScrollView {
					Text(
"""
This tiny app demonstrates how Sargon written in Rust can be used in an iOS app, thanks to the Swift bindings that we have generated with UniFFI.

The build artifacts of UniFFI are have three major components:
1) A set of binaries we have grouped to together with lipo and put in a .xcframework vendored as a binaryTarget in the Sargon Swift Package.

2) A single HUGE Swift file with Swift models exported from Rust and with function pointers that use the binaryTarget in the Sargon Swift Package

3) A set of Swift extension's on the bindgen generated Swift models, e.g. making `Decimal192` conform to `ExpressibleByIntegerLiteral` which of course is a pure Swift construct. Also marking all types as `Sendable` and `CustomStringConvertible` making use of their `std::fmt::Display` impl in Rust land.
"""
					)
					.padding()
				}
				Button("Start") {
					send(.continueButtonTapped)
				}
			}
			.padding()
		}
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.continueButtonTapped):
				.send(.delegate(.done))
			case .delegate:
				.none
			}
		}
	}
}
