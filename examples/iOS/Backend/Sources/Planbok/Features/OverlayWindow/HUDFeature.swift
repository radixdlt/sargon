//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-11.
//

import Foundation
import ComposableArchitecture

@Reducer
public struct HUDFeature {
    
    @Dependency(\.continuousClock) var clock
    
    @ObservableState
    public struct State: Equatable {
        static let hiddenOffset: CGFloat = -128.0
        static let autoDismissDelay: Double = 1.0

        let content: HUDMessage
        var offset = Self.hiddenOffset
        
        var text: String {
            content.text
        }
        var icon: HUDMessage.Icon? {
            content.icon
        }
    }
    
    public enum Action: ViewAction, Sendable {
        public enum ViewAction: Sendable {
            case onAppear
            case animationCompletion
        }
        public enum DelegateAction: Sendable {
            case dismiss
        }
        public enum InternalAction: Sendable {
            case autoDismiss
        }
        case view(ViewAction)
        case delegate(DelegateAction)
        case `internal`(InternalAction)
    }
    
    public var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
            case .internal(.autoDismiss):
                state.offset = State.hiddenOffset
                return .none
            case .view(.animationCompletion):
                if state.offset == State.hiddenOffset {
                    /// Notify the delegate only after the animation did complete.
                    return .send(.delegate(.dismiss))
                } else {
                    return .run { send in
                        try await clock.sleep(for: .seconds(State.autoDismissDelay))
                        await send(.internal(.autoDismiss), animation: .hudAnimation)
                    }
                }
            case .view(.onAppear):
                state.offset = 0
                return .none
            case .delegate:
                return .none
            }
        }
    }
}

extension HUDFeature {
    public typealias HostingFeature = Self
    
    @ViewAction(for: HostingFeature.self)
    public struct View: SwiftUI.View {
        public let store: StoreOf<HostingFeature>
        public init(store: StoreOf<HostingFeature>) {
            self.store = store
        }
        public var body: some SwiftUI.View {
            VStack {
                HStack {
                    if let icon = store.icon {
                        Image(systemName: icon.systemName)
                        .foregroundColor(icon.foregroundColor)
                        .frame(width: 25, height: 25)
                    }
                    
                    Text(store.text)
                        .foregroundColor(.app.gray1)
                        .font(.footnote)
                }
                .padding(.vertical, .small1)
                .padding(.horizontal, .medium3)
                .background(
                    Capsule()
                        .foregroundColor(.app.background)
                        .shadow(
                            color: .app.gray1.opacity(0.16),
                            radius: 12,
                            x: 0,
                            y: 5
                        )
                )
                .offset(y: store.offset)
                .onAppear {
                    send(.onAppear, animation: .hudAnimation)
                }
                .onAnimationCompleted(for: store.offset) {
                    send(.animationCompletion)
                }
                
                Spacer()
            }
        }
    }
}


// MARK: - SwiftUI.Animation + Sendable
extension SwiftUI.Animation: @unchecked Sendable {}

extension SwiftUI.Animation {
    static var hudAnimation: SwiftUI.Animation {
        .spring()
    }
}

extension View {
    public func onAnimationCompleted(
        for animatedValue: some Sendable & VectorArithmetic,
        completion: @escaping @Sendable () -> Void
    ) -> some View {
        modifier(OnAnimationCompletedViewModifier(animatedValue: animatedValue, completion: completion))
    }
}
// MARK: - OnAnimationCompletedViewModifier
/// A view modifier allowing to observe the completion of a given value animation
private struct OnAnimationCompletedViewModifier<Value: Sendable & VectorArithmetic>: Animatable, ViewModifier {
    typealias Completion = @MainActor @Sendable () -> Void
    var animatableData: Value {
        didSet {
            guard animatableData == animatedValue else { return }
            Task { [completion] in
                await completion()
            }
        }
    }

    private let animatedValue: Value
    private let completion: Completion

    init(animatedValue: Value, completion: @escaping Completion) {
        self.animatedValue = animatedValue
        self.animatableData = animatedValue
        self.completion = completion
    }

    func body(content: Content) -> some View {
        content
    }
}
