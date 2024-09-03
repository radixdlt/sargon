package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.ProfileStateChangeDriver
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.filterIsInstance
import kotlinx.coroutines.flow.map

class AndroidProfileStateChangeDriver : ProfileStateChangeDriver {

    private val _profileStateChanges = MutableSharedFlow<ProfileState>()

    val profileState: SharedFlow<ProfileState> = _profileStateChanges.asSharedFlow()
    val profile: Flow<Profile> = profileState.filterIsInstance<ProfileState.Loaded>().map {
        it.v1
    }

    override suspend fun handleProfileStateChange(changedProfileState: ProfileState) {
        _profileStateChanges.emit(changedProfileState)
    }
}