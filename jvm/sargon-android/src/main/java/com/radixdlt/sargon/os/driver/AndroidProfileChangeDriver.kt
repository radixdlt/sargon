package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileChangeDriver
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow

class AndroidProfileChangeDriver : ProfileChangeDriver {

    private val _profile = MutableSharedFlow<Profile>()
    val profile: Flow<Profile> = _profile.asSharedFlow()

    override suspend fun handleProfileChange(changedProfile: Profile) {
        _profile.emit(changedProfile)
    }
}