package com.radixdlt.sargon.os.driver

import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.ProfileStateChangeDriver
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.filterIsInstance
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update

object AndroidProfileStateChangeDriver : ProfileStateChangeDriver {

    private val _profileState: MutableStateFlow<ProfileState?> = MutableStateFlow(null)

    val profileState: StateFlow<ProfileState?> = _profileState.asStateFlow()
    val profile: Flow<Profile> = _profileState
        .filterIsInstance<ProfileState.Loaded>()
        .map { it.v1 }

    override suspend fun handleProfileStateChange(changedProfileState: ProfileState) {
        _profileState.update { changedProfileState }
    }
}