//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI
import AsyncExtensions

extension EventBusDriver where Self == EventBus {
	public static var shared: Self { Self.shared }
}

public final actor EventBus {
	private let stream = AsyncThrowingPassthroughSubject<Element, any Error>()
	private let subject: Subject
	public init() {
		subject = .init()
	}
}

extension EventBus {
	
	public typealias Element = EventNotification
	public typealias Subject = AsyncPassthroughSubject<Element>
	
	public static let shared = EventBus()
	
	public func notifications() -> AsyncMulticastSequence<EventBus.Subject, AsyncThrowingPassthroughSubject<EventBus.Element, any Error>> {
		subject
		 .multicast(stream)
		 .autoconnect()
	}
}

extension EventBus: EventBusDriver {
	public func handleEventNotification(eventNotification: EventNotification) async {
		log.debug("Handle event: \(String(describing: eventNotification.event))")
		subject.send(eventNotification)
	}
}
