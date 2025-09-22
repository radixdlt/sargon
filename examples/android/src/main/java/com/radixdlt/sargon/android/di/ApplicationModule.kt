package com.radixdlt.sargon.android.di

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.preferencesDataStore
import com.radixdlt.sargon.ArculusCsdkDriver
import com.radixdlt.sargon.ArculusVerifyPinResponse
import com.radixdlt.sargon.ArculusWalletPointer
import com.radixdlt.sargon.AuthorizationPurpose
import com.radixdlt.sargon.AuthorizationResponse
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.NfcTagDriver
import com.radixdlt.sargon.NfcTagDriverPurpose
import com.radixdlt.sargon.SignRequestOfAuthIntent
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash
import com.radixdlt.sargon.SpotCheckResponse
import com.radixdlt.sargon.android.BuildConfig
import com.radixdlt.sargon.os.SargonOsManager
import com.radixdlt.sargon.os.driver.AndroidEventBusDriver
import com.radixdlt.sargon.os.driver.AndroidProfileStateChangeDriver
import com.radixdlt.sargon.os.driver.BiometricsHandler
import com.radixdlt.sargon.os.from
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import javax.inject.Qualifier
import javax.inject.Singleton

@Retention(AnnotationRetention.RUNTIME)
@Qualifier
annotation class EncryptedPreferences

@Retention(AnnotationRetention.RUNTIME)
@Qualifier
annotation class NonEncryptedPreferences

@Retention(AnnotationRetention.RUNTIME)
@Qualifier
annotation class DeviceInfoPreferences

@Retention(AnnotationRetention.BINARY)
@Qualifier
annotation class DefaultDispatcher

@Retention(AnnotationRetention.BINARY)
@Qualifier
annotation class IoDispatcher

@Retention(AnnotationRetention.BINARY)
@Qualifier
annotation class ApplicationScope

@Module
@InstallIn(SingletonComponent::class)
object ApplicationModule {

    private val Context.preferencesDatastore: DataStore<Preferences> by preferencesDataStore(
        name = "example_preferences"
    )

    private val Context.encryptedPreferencesDatastore: DataStore<Preferences> by preferencesDataStore(
        name = "example_encrypted_preferences"
    )

    private val Context.deviceInfoPreferencesDatastore: DataStore<Preferences> by preferencesDataStore(
        name = "example_device_info_preferences"
    )

    @DefaultDispatcher
    @Provides
    fun providesDefaultDispatcher(): CoroutineDispatcher = Dispatchers.Default

    @IoDispatcher
    @Provides
    fun providesIoDispatcher(): CoroutineDispatcher = Dispatchers.IO

    @Singleton
    @ApplicationScope
    @Provides
    fun providesCoroutineScope(
        @DefaultDispatcher defaultDispatcher: CoroutineDispatcher
    ): CoroutineScope = CoroutineScope(SupervisorJob() + defaultDispatcher)


    @Provides
    @Singleton
    fun provideHttpLoggingInterceptor(): HttpLoggingInterceptor {
        return HttpLoggingInterceptor().apply {
            level = if (BuildConfig.DEBUG) {
                HttpLoggingInterceptor.Level.BODY
            } else {
                HttpLoggingInterceptor.Level.NONE
            }
        }
    }

    @Provides
    @Singleton
    fun provideGatewayHttpClient(
        httpLoggingInterceptor: HttpLoggingInterceptor,
    ): OkHttpClient {
        return OkHttpClient.Builder()
            .addInterceptor(httpLoggingInterceptor)
            .build()
    }

    @Provides
    @Singleton
    @NonEncryptedPreferences
    fun providePreferences(
        @ApplicationContext context: Context
    ): DataStore<Preferences> = context.preferencesDatastore

    @Provides
    @Singleton
    @EncryptedPreferences
    fun provideEncryptedPreferences(
        @ApplicationContext context: Context
    ): DataStore<Preferences> = context.encryptedPreferencesDatastore

    @Provides
    @Singleton
    @DeviceInfoPreferences
    fun provideDeviceInfoPreferences(
        @ApplicationContext context: Context
    ): DataStore<Preferences> = context.deviceInfoPreferencesDatastore

    @Provides
    @Singleton
    fun provideBiometricsHandler(): BiometricsHandler = BiometricsHandler(
        biometricsSystemDialogTitle = "Authenticate to continue"
    )

    @Provides
    @Singleton
    fun provideEventBusDriver(): AndroidEventBusDriver = AndroidEventBusDriver

    @Provides
    @Singleton
    fun provideProfileStateChangeDriver(): AndroidProfileStateChangeDriver =
        AndroidProfileStateChangeDriver

    object HostInteractorStub : HostInteractor {

        override suspend fun signTransactions(request: SignRequestOfTransactionIntent): SignResponseOfTransactionIntentHash {
            throw Exception("Not yet implemented")
        }

        override suspend fun signSubintents(request: SignRequestOfSubintent): SignResponseOfSubintentHash {
            throw Exception("Not yet implemented")
        }

        override suspend fun deriveKeys(request: KeyDerivationRequest): KeyDerivationResponse {
            throw Exception("Not yet implemented")
        }

        override suspend fun signAuth(request: SignRequestOfAuthIntent): SignResponseOfAuthIntentHash {
            throw Exception("Not yet implemented")
        }

        override suspend fun requestAuthorization(purpose: AuthorizationPurpose): AuthorizationResponse {
            throw Exception("Not yet implemented")
        }

        override suspend fun spotCheck(factorSource: FactorSource, allowSkip: Boolean): SpotCheckResponse {
            throw Exception("Not yet implemented")
        }

    }

    @Provides
    @Singleton
    fun provideBios(
        @ApplicationContext context: Context,
        httpClient: OkHttpClient,
        eventBusDriver: AndroidEventBusDriver,
        profileStateChangeDriver: AndroidProfileStateChangeDriver,
        biometricsHandler: BiometricsHandler,
        @EncryptedPreferences encryptedPreferences: DataStore<Preferences>,
        @NonEncryptedPreferences preferences: DataStore<Preferences>,
        @DeviceInfoPreferences deviceInfoPreferences: DataStore<Preferences>,
    ): Bios = Bios.from(
        context = context,
        httpClient = httpClient,
        biometricsHandler = biometricsHandler,
        encryptedPreferencesDataStore = encryptedPreferences,
        preferencesDatastore = preferences,
        deviceInfoDatastore = deviceInfoPreferences,
        eventBusDriver = eventBusDriver,
        profileStateChangeDriver = profileStateChangeDriver,
        arculusCsdkDriver = FakeArculusCsdkDriver(),
        nfcTagDriver = FakeNfcTagDriver()
    )

    @Provides
    @Singleton
    fun provideSargonOsManager(
        bios: Bios,
        @ApplicationScope applicationScope: CoroutineScope,
        @DefaultDispatcher dispatcher: CoroutineDispatcher
    ): SargonOsManager = SargonOsManager.factory(
        bios = bios,
        applicationScope = applicationScope,
        defaultDispatcher = dispatcher,
        hostInteractor = HostInteractorStub
    )
}

class FakeArculusCsdkDriver : ArculusCsdkDriver {

    override fun walletInit(): ArculusWalletPointer? {
        TODO("Not yet implemented")
    }

    override fun walletFree(wallet: ArculusWalletPointer) {
        TODO("Not yet implemented")
    }

    override fun selectWalletRequest(
        wallet: ArculusWalletPointer,
        aid: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun selectWalletResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun createWalletSeedRequest(
        wallet: ArculusWalletPointer,
        wordCount: Long
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun createWalletSeedResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun seedPhraseFromMnemonicSentence(
        wallet: ArculusWalletPointer,
        mnemonicSentence: BagOfBytes,
        passphrase: BagOfBytes?
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun initRecoverWalletRequest(
        wallet: ArculusWalletPointer,
        wordCount: Long
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun initRecoverWalletResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): Int {
        TODO("Not yet implemented")
    }

    override fun finishRecoverWalletRequest(
        wallet: ArculusWalletPointer,
        seed: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun finishRecoverWalletResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): Int {
        TODO("Not yet implemented")
    }

    override fun resetWalletRequest(wallet: ArculusWalletPointer): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun resetWalletResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): Int {
        TODO("Not yet implemented")
    }

    override fun getGguidRequest(wallet: ArculusWalletPointer): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun getGguidResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun getFirmwareVersionRequest(wallet: ArculusWalletPointer): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun getFirmwareVersionResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun storeDataPinRequest(
        wallet: ArculusWalletPointer,
        pin: String
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun storeDataPinResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): Int {
        TODO("Not yet implemented")
    }

    override fun verifyPinRequest(
        wallet: ArculusWalletPointer,
        pin: String
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun verifyPinResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): ArculusVerifyPinResponse {
        TODO("Not yet implemented")
    }

    override fun initEncryptedSessionRequest(wallet: ArculusWalletPointer): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun initEncryptedSessionResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): Int {
        TODO("Not yet implemented")
    }

    override fun getPublicKeyByPathRequest(
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: UShort
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun getPublicKeyByPathResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }

    override fun signHashPathRequest(
        wallet: ArculusWalletPointer,
        path: BagOfBytes,
        curve: UShort,
        algorithm: UByte,
        hash: BagOfBytes
    ): List<BagOfBytes>? {
        TODO("Not yet implemented")
    }

    override fun signHashPathResponse(
        wallet: ArculusWalletPointer,
        response: BagOfBytes
    ): BagOfBytes? {
        TODO("Not yet implemented")
    }
}

class FakeNfcTagDriver : NfcTagDriver {

    override suspend fun startSession(purpose: NfcTagDriverPurpose) {
        TODO("Not yet implemented")
    }

    override suspend fun endSession(withFailure: CommonException?) {
        TODO("Not yet implemented")
    }

    override suspend fun sendReceive(command: BagOfBytes): BagOfBytes {
        TODO("Not yet implemented")
    }

    override suspend fun sendReceiveCommandChain(commands: List<BagOfBytes>): BagOfBytes {
        TODO("Not yet implemented")
    }

    override suspend fun setProgress(progress: UByte) {
        TODO("Not yet implemented")
    }

}