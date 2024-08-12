package com.radixdlt.sargon.android.di

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.preferencesDataStore
import com.radixdlt.sargon.Bios
import com.radixdlt.sargon.android.BuildConfig
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
    fun provideBios(
        @ApplicationContext context: Context,
        httpClient: OkHttpClient,
        biometricsHandler: BiometricsHandler,
        @EncryptedPreferences encryptedPreferences: DataStore<Preferences>,
        @NonEncryptedPreferences preferences: DataStore<Preferences>,
        @DeviceInfoPreferences deviceInfoPreferences: DataStore<Preferences>,
    ): Bios = Bios.from(
        context = context,
        enableLogging = BuildConfig.DEBUG,
        httpClient = httpClient,
        biometricsHandler = biometricsHandler,
        encryptedPreferencesDataStore = encryptedPreferences,
        preferencesDatastore = preferences,
        deviceInfoDatastore = deviceInfoPreferences
    )
}