package com.radixdlt.sargon.serializer

import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.extensions.bagOfBytes
import com.radixdlt.sargon.extensions.string
import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder

/**
 * A class that aims to bridge the serialization logic between Rust (serde) and kotlinx
 * serialization.
 *
 * This kind of serializer offloads the serialization/deserialization logic into sargon
 * and plugs in the result into the `KSerializer` as a `JsonElement`.
 */
abstract class KotlinRustSerializer<T>(serialName: String): KSerializer<T> {

    abstract fun rustFunctionFromJsonBytes(
        jsonBytes: BagOfBytes
    ): T

    abstract fun rustFunctionToJsonBytes(
        value: T
    ): BagOfBytes

    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor(
        serialName = serialName,
        kind = PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): T {
        val jsonDecoder = decoder as JsonDecoder
        val json = jsonDecoder.decodeJsonElement().toString()
        return rustFunctionFromJsonBytes(bagOfBytes(fromString = json))
    }

    override fun serialize(encoder: Encoder, value: T) {
        val jsonEncoder = encoder as JsonEncoder

        val jsonElement = Json.parseToJsonElement(
            rustFunctionToJsonBytes(value = value).string
        )
        jsonEncoder.encodeJsonElement(jsonElement)
    }

}