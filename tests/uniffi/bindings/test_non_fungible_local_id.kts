import com.radixdlt.sargon.*

val NonFungibleLocalId.description: String
    get() = nonFungibleLocalIdAsStr(id = this)

fun String.hexToByteArray(): ByteArray {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2).map { it.toInt(16).toByte() }.toByteArray()
}

fun ByteArray.toBagOfBytes() = newBagOfBytesFrom(bytes = this)

fun Exactly32Bytes.Companion.init(bytes: ByteArray): Exactly32Bytes = newExactly32Bytes(bytes = bytes.toBagOfBytes())


fun test() {
    val hex = "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210"
    val bagOfBytes = hex.hexToByteArray().toBagOfBytes()
    assert(NonFungibleLocalId.Integer(value = 1234.toULong()).description == "#1234#")
    assert(newNonFungibleLocalIdString(string = "foo").description == "<foo>")
    assert(
            newNonFungibleLocalIdRuid(bytes = bagOfBytes).description ==
                    "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
    )
    assert(
            newNonFungibleLocalIdBytes(bytes = bagOfBytes).description ==
                    "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]"
    )
}

test()
