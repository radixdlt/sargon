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
		@Dependency(PasteboardClient.self) var pasteboardClient
        let items = AsyncPassthroughSubject<HUDMessage>()
		
		let scheduleHUDMessage: ScheduleHUDMessage = { message in
			items.send(message)
		}
		
		Task {
			for await event in await EventBus.shared.notifications() {
				scheduleHUDMessage(
					HUDMessage(
						text: "Sargon Event: `\(event.event.kind)`",
						icon: HUDMessage.Icon(
							systemName: "bell",
							foregroundColor: .blue
						)
					)
				)
			}
		}
		
		Task {
			for try await _ in pasteboardClient.copyEvents() {
				scheduleHUDMessage(
					HUDMessage.success(text: "Copied")
				)

			}
		}
        
        return Self(
            getScheduledItems: { items.eraseToAnyAsyncSequence() },
            scheduleHUDMessage: scheduleHUDMessage
        )
    }()
    
    public static let testValue = OverlayWindowClient(
        getScheduledItems: { AsyncLazySequence([]).eraseToAnyAsyncSequence() },
        scheduleHUDMessage: { _ in }
    )
}

extension HUDMessage {
	public static let openedSecurityQuestionsSealedMnemonic = Self.success(text: "Successful decryption with answers.")
	public static let failedToOpenSecurityQuestionsSealedMnemonic = Self.failure(text: "Failed to decrypt with answers.")
}
