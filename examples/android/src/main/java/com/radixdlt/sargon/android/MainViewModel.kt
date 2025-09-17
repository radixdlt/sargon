package com.radixdlt.sargon.android

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.radixdlt.sargon.Account
import com.radixdlt.sargon.DeviceFactorSource
import com.radixdlt.sargon.DisplayName
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.FactorSourceKind
import com.radixdlt.sargon.HostId
import com.radixdlt.sargon.HostInfo
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.Profile
import com.radixdlt.sargon.ProfileState
import com.radixdlt.sargon.Timestamp
import com.radixdlt.sargon.extensions.kind
import com.radixdlt.sargon.os.SargonOsManager
import com.radixdlt.sargon.os.SargonOsState
import com.radixdlt.sargon.os.driver.AndroidProfileStateChangeDriver
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import timber.log.Timber
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(
    private val sargonOsManager: SargonOsManager,
    private val profileStateChangeDriver: AndroidProfileStateChangeDriver
) : ViewModel() {

    val state = combine(
        sargonOsManager.sargonState,
        profileStateChangeDriver.profileState,
    ) { sargonState, profileState ->
        State(
            sargonState,
            profileState
        )
    }.map { state ->
        if (state.info == null) {
            val osState = state.sargonState as? SargonOsState.Booted ?: return@map state

            state.copy(
                info = HostInformation(
                    id = osState.os.hostId(),
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
            val os = sargonOsManager.sargonOs
            runCatching {
                os.newWallet()
            }.onFailure { error ->
                Timber.tag("sargon app").w(error)
            }
        }
    }

    fun onImportWallet(profile: Profile) = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs
            runCatching {
                os.importWallet(profile = profile, bdfsSkipped = true)
            }.onFailure { error ->
                Timber.tag("sargon app").w(error)
            }
        }
    }

    fun onDeleteWallet() = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs
            runCatching {
                os.deleteWallet()
            }.onFailure { error ->
                Timber.tag("sargon app").w(error)
            }
        }
    }

    fun onCreateAccountWithDevice(
        networkId: NetworkId,
        accountName: String
    ) = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs
            val bdfs = os.factorSources().first { it.kind == FactorSourceKind.DEVICE }

            runCatching {
                os.createAndSaveNewAccountWithFactorSource(
                    networkId = networkId,
                    name = DisplayName(accountName),
                    factorSource = bdfs
                )
            }.onFailure { error ->
                Timber.tag("sargon app").w(error)
            }
        }
    }

    fun onDevModeChanged(enabled: Boolean) = viewModelScope.launch {
        withContext(Dispatchers.Default) {
            val os = sargonOsManager.sargonOs
            val profile = profileStateChangeDriver.profile.first()

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
        val sargonState: SargonOsState = SargonOsState.Idle,
        val profileState: ProfileState? = null,
        val accounts: List<Account> = emptyList(),
        val info: HostInformation? = null
    )

    data class HostInformation(
        val id: HostId,
        val info: HostInfo
    )

}