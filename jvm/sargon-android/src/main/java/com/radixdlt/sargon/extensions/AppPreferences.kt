package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.FfiUrl
import com.radixdlt.sargon.appPreferencesHasP2pTransportProfileWithSignalingServerUrl
import com.radixdlt.sargon.newAppPreferencesDefault

fun AppPreferences.Companion.default() = newAppPreferencesDefault()

fun AppPreferences.hasP2pTransportProfileWithSignalingServerUrl(url: FfiUrl): Boolean =
    appPreferencesHasP2pTransportProfileWithSignalingServerUrl(
        appPreferences = this,
        url = url
    )
