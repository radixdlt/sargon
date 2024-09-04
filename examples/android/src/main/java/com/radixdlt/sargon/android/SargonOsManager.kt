package com.radixdlt.sargon.android

import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.SargonOs
import com.radixdlt.sargon.android.di.ApplicationScope
import com.radixdlt.sargon.os.driver.AndroidProfileStateChangeDriver
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.filterIsInstance
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class SargonOsManager @Inject constructor(
    private val bios: Bios,
    private val profileStateChangeDriver: AndroidProfileStateChangeDriver,
    @ApplicationScope private val applicationScope: CoroutineScope
) {

    private val _sargonState = MutableStateFlow<SargonState>(SargonState.Idle)
    private val _profileState = MutableStateFlow<ProfileState>(ProfileState.NotInitialised)

    val sargonState: StateFlow<SargonState>
        get() = _sargonState.asStateFlow()

    val sargonOs: Flow<SargonOs> = sargonState
        .filterIsInstance<SargonState.Booted>()
        .map { it.os }

    val profileState: StateFlow<ProfileState> = _profileState.asStateFlow()
    val profile: Flow<Profile> = profileState
        .filterIsInstance<ProfileState.Restored>()
        .map { it.profile }

    init {
        applicationScope.launch {
            profileStateChangeDriver
                .profileState
                .map { state ->
                    when (state) {
                        is com.radixdlt.sargon.ProfileState.None -> ProfileState.None
                        is com.radixdlt.sargon.ProfileState.Incompatible -> ProfileState.Incompatible(cause = state.v1)
                        is com.radixdlt.sargon.ProfileState.Loaded -> ProfileState.Restored(profile = state.v1)
                    }
                }.collect { state ->
                    _profileState.update { state }
                }
        }

        applicationScope.launch {
            withContext(Dispatchers.Default) {
                val os = SargonOs.boot(bios)
                _sargonState.update { SargonState.Booted(os) }
            }
        }
    }

    sealed interface SargonState {
        data object Idle: SargonState
        data class Booted(
            val os: SargonOs
        ): SargonState
    }

    sealed interface ProfileState {
        data object NotInitialised : ProfileState
        data object None : ProfileState
        data class Incompatible(val cause: Throwable) : ProfileState
        data class Restored(val profile: Profile) : ProfileState {
            fun hasNetworks(): Boolean {
                return profile.networks.isNotEmpty()
            }
        }
    }

}