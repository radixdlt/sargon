package com.radixdlt.sargon.android

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.radixdlt.sargon.Account
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.Timestamp
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(
    private val sargonOsManager: SargonOsManager
) : ViewModel() {

    val state = combine(
        sargonOsManager.sargonState,
        sargonOsManager.profileState,
    ) { sargonState, profileState ->
        State(
            sargonState,
            profileState
        )
    }.map { state ->
        if (state.info == null) {
            val osState = state.sargonState as? SargonOsManager.SargonState.Booted ?: return@map state

            state.copy(
                info = HostInformation(
                    id = osState.os.resolveHostId(),
                    info = osState.os.resolveHostInfo()
                )
            )
        } else {
            state
        }
    }.stateIn(
        scope = viewModelScope,
        started = SharingStarted.Eagerly,
        initialValue = State()
    )

    fun onCreateNewWallet() = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs.first()
            runCatching {
                os.newWallet()
            }
        }
    }

    fun onImportWallet(profile: Profile) = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs.first()
            runCatching {
                os.importWallet(profile = profile, bdfsSkipped = true)
            }
        }
    }

    fun onDeleteWallet() = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs.first()
            runCatching {
                os.deleteWallet()
            }
        }
    }

    fun onDevModeChanged(enabled: Boolean) = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs.first()
            val profile = sargonOsManager.profile.first()

            runCatching {
                os.setProfile(profile.mutate {
                    it.copy(
                        appPreferences = it.appPreferences.copy(
                            security = it.appPreferences.security.copy(
                                isDeveloperModeEnabled = enabled
                            )
                        )
                    )
                })
            }
        }
    }

    private fun Profile.mutate(mutation: (Profile) -> Profile): Profile {
        val mutated = mutation(this)

        return mutated.copy(
            header = mutated.header.copy(
                lastModified = Timestamp.now()
            )
        )
    }

    data class State(
        val sargonState: SargonOsManager.SargonState = SargonOsManager.SargonState.Idle,
        val profileState: ProfileState = ProfileState.None,
        val accounts: List<Account> = emptyList(),
        val info: HostInformation? = null
    )

    data class HostInformation(
        val id: HostId,
        val info: HostInfo
    )

}