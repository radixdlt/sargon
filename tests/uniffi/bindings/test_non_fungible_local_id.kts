import com.radixdlt.sargon.*

val NonFungibleLocalId.description: String
    get() = nonFungibleLocalIdAsStr(id = this)

fun String.hexToByteArray(): ByteArray {
    check(length % 2 == 0) { "Must have an even length" }

    return chunked(2)
        .map { it.toInt(16).toByte() }
        .toByteArray()
}

fun test() {
    val hex = "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210"
	val bagOfBytes = newBagOfBytesFrom(bytes = hex.hexToByteArray())
	assert(NonFungibleLocalId.Integer(value = 1234.toULong()).description == "#1234#")
	assert(NonFungibleLocalId.Str(value = "foo").description == "<foo>")
	assert(
	    NonFungibleLocalId.Ruid(value = bagOfBytes).description
	        == "{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}"
    )
	assert(
		NonFungibleLocalId.Bytes(value = bagOfBytes).description
			== "[deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210]"
    )

}

test()
