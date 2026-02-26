package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.appPreferencesHasP2pTransportProfileWithSignalingServer
import com.radixdlt.sargon.newAppPreferencesDefault

fun AppPreferences.Companion.default() = newAppPreferencesDefault()

fun AppPreferences.hasP2pTransportProfileWithSignalingServer(signalingServer: String): Boolean =
    appPreferencesHasP2pTransportProfileWithSignalingServer(
        appPreferences = this,
        signalingServer = signalingServer
    )
