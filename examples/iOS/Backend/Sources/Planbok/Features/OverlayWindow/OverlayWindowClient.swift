//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-11.
//

import Foundation
import SwiftUI
import Dependencies
import ComposableArchitecture
import Sargon
import AsyncExtensions

@DependencyClient
public struct OverlayWindowClient: Sendable {
    public typealias GetScheduledItems = @Sendable () -> AnyAsyncSequence<HUDMessage>
    public typealias ScheduleHUDMessage = @Sendable (HUDMessage) -> Void

    public var getScheduledItems: GetScheduledItems
    
    /// Schedule a HUD message to be shown in the Overlay Window.
    /// Usually to be called from the Main Window.
    public var scheduleHUDMessage: ScheduleHUDMessage

}

extension OverlayWindowClient: DependencyKey {
    public static let liveValue: Self = {
        let items = AsyncPassthroughSubject<HUDMessage>()
        
        return Self(
            getScheduledItems: { items.eraseToAnyAsyncSequence() },
            scheduleHUDMessage: { message in
                items.send(message)
            }
        )
    }()
    
    public static let testValue = OverlayWindowClient(
        getScheduledItems: { AsyncLazySequence([]).eraseToAnyAsyncSequence() },
        scheduleHUDMessage: { _ in }
    )
}

extension HUDMessage {
    public static let openedSecurityQuestionsSealedMnemonic = Self(text: "Successful decryption with answers.")
}
