//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-12.
//

import Foundation
import Dependencies
import AsyncExtensions

#if canImport(UIKit)
import UIKit
public typealias Pasteboard = UIPasteboard
extension Pasteboard {
	@Sendable public func setCopied(string: String) {
		self.string = string
	}
	@Sendable public func getCopied() -> String? {
		self.string
	}
}
#elseif canImport(AppKit)
import AppKit
public typealias Pasteboard = NSPasteboard
extension NSPasteboard: @unchecked Sendable {}

extension Pasteboard {
	@Sendable public func setCopied(string: String) {
		self.setString(string, forType: .string)
	}
	@Sendable public func getCopied() -> String? {
		self.string(forType: .string)
	}
}
#endif

@DependencyClient
public struct PasteboardClient: Sendable {
	public typealias CopyEvents = @Sendable () -> AnyAsyncSequence<String>
	public typealias CopyString = @Sendable (String) -> Void
	public typealias GetString = @Sendable () -> String?
	
	public var copyEvents: CopyEvents
	public var copyString: CopyString
	public var getString: GetString
}
extension PasteboardClient: DependencyKey {
	public typealias Value = Self
	public static let liveValue = Self.live()
	static func live(pasteboard: Pasteboard = .general) -> Self {
		let copyEvents = AsyncPassthroughSubject<String>()

		return Self(
			copyEvents: { copyEvents.share().eraseToAnyAsyncSequence() },
			copyString: { aString in
				pasteboard.setCopied(string: aString)
				copyEvents.send(aString)
			},
			getString: {
				pasteboard.getCopied()
			}
		)
	}
}
