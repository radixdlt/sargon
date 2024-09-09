package com.radixdlt.sargon.android

import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
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

    val sargonState: StateFlow<SargonState>
        get() = _sargonState.asStateFlow()

    val profileState: StateFlow<ProfileState?> = profileStateChangeDriver.profileState
    val profile: Flow<Profile> = profileStateChangeDriver.profile

    val sargonOs: Flow<SargonOs> = sargonState
        .filterIsInstance<SargonState.Booted>()
        .map { it.os }

    init {
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

}