package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.ResourcePreferenceKind
import com.radixdlt.sargon.ResourcePreferences
import com.radixdlt.sargon.resourcePreferencesGetHiddenResources
import com.radixdlt.sargon.resourcePreferencesHideResource
import com.radixdlt.sargon.resourcePreferencesUnhideResource

val ResourcePreferences.hiddenResources
    get() = resourcePreferencesGetHiddenResources(this)

fun ResourcePreferences.hide(kind: ResourcePreferenceKind) = resourcePreferencesHideResource(this, kind)

fun ResourcePreferences.unhide(kind: ResourcePreferenceKind) = resourcePreferencesUnhideResource(this, kind)