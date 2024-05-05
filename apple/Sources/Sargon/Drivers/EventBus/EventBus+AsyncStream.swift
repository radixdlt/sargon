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

public final actor EventBus {
	public typealias Element = Event
	public typealias Stream = AsyncStream<Element>
	
	private let continuation: Stream.Continuation
	private let stream: Stream
	
	public init() {
		(stream, continuation) = Stream.makeStream()
	}
	
	public static let shared = EventBus()
}

extension EventBus {
	public func events() -> Stream {
		stream
	}
}

extension EventBus: EventBusDriver {
	public func handleEvent(event: Event) async {
		continuation.yield(event)
	}
}
