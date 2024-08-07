package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.EventBusDriver
import com.radixdlt.sargon.EventNotification
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow

class AndroidEventBusDriver: EventBusDriver {

    private val _events = MutableSharedFlow<EventNotification>()
    val events: Flow<EventNotification> = _events.asSharedFlow()

    override suspend fun handleEventNotification(eventNotification: EventNotification) {
        _events.emit(eventNotification)
    }
}