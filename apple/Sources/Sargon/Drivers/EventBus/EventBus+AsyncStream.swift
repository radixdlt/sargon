//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension EventBusDriver where Self == EventBus {
	public static var shared: Self { Self.shared }
}


extension Event {
	public var addressOfNewAccount: AccountAddress {
		switch self {
		case let .profileChanged(change: .addedAccount(address)): return address
		}
	}
}

extension EventNotification: Comparable {
	public static func < (lhs: Self, rhs: Self) -> Bool {
		lhs.timestamp < rhs.timestamp
	}
}


public final actor EventBus {
	public typealias Element = EventNotification
	public typealias Stream = AsyncStream<Element>
	
	private let continuation: Stream.Continuation
	private let stream: Stream
	
	public init() {
		(stream, continuation) = Stream.makeStream()
	}
	
	public static let shared = EventBus()
}

extension EventBus {
	public func notifications() -> Stream {
		stream
	}
}

extension EventBus: EventBusDriver {
	public func handleEventNotification(eventNotification: EventNotification) async {
		log.debug("Handle event: \(String(describing: eventNotification.event))")
		continuation.yield(eventNotification)
	}
}
