//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-11.
//

import Foundation

public struct HUDMessage: Sendable, Hashable, Identifiable {
    public let id = UUID()
    public let text: String
    public let icon: Icon?

    public struct Icon: Hashable, Sendable {
        public let systemName: String
        public let foregroundColor: Color

        public init(
            systemName: String,
            foregroundColor: Color = .app.green1
        ) {
            self.systemName = systemName
            self.foregroundColor = foregroundColor
        }
    }

    public init(
        text: String,
        icon: Icon? = Icon(
            systemName: "checkmark.circle.fill",
            foregroundColor: Color.app.green1
        )
    ) {
        self.text = text
        self.icon = icon
    }
}
