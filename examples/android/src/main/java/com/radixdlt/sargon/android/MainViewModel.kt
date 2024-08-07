package com.radixdlt.sargon.android

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.radixdlt.sargon.Account
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(
    private val sargonOsManager: SargonOsManager
) : ViewModel() {

    val state = sargonOsManager.sargonState
        .map {
            State(sargonState = it)
        }
        .stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(),
            initialValue = State()
        )

    fun retryBooting() {
        viewModelScope.launch {
            sargonOsManager.boot()
        }
    }

    data class State(
        val sargonState: SargonOsManager.SargonState = SargonOsManager.SargonState.Idle,
        val accounts: List<Account> = emptyList()
    )

}