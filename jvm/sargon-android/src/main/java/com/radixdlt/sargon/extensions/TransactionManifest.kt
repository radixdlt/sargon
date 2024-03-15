package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.AccountAddress
import com.radixdlt.sargon.AddressOfAccountOrPersona
import com.radixdlt.sargon.Blobs
import com.radixdlt.sargon.Decimal192
import com.radixdlt.sargon.NetworkId
import com.radixdlt.sargon.PerAssetTransfers
import com.radixdlt.sargon.PerRecipientAssetTransfers
import com.radixdlt.sargon.PublicKeyHash
import com.radixdlt.sargon.StakeClaim
import com.radixdlt.sargon.ThirdPartyDeposits
import com.radixdlt.sargon.TokenDefinitionMetadata
import com.radixdlt.sargon.TransactionGuarantee
import com.radixdlt.sargon.TransactionManifest
import com.radixdlt.sargon.manifestCreateFungibleToken
import com.radixdlt.sargon.manifestCreateFungibleTokenWithMetadata
import com.radixdlt.sargon.manifestCreateMultipleFungibleTokens
import com.radixdlt.sargon.manifestCreateMultipleNonFungibleTokens
import com.radixdlt.sargon.manifestCreateNonFungibleToken
import com.radixdlt.sargon.manifestForFaucet
import com.radixdlt.sargon.manifestMarkingAccountAsDappDefinitionType
import com.radixdlt.sargon.manifestPerAssetTransfers
import com.radixdlt.sargon.manifestPerRecipientTransfers
import com.radixdlt.sargon.manifestSetOwnerKeysHashes
import com.radixdlt.sargon.manifestStakesClaim
import com.radixdlt.sargon.manifestThirdPartyDepositUpdate
import com.radixdlt.sargon.modifyManifestAddGuarantees
import com.radixdlt.sargon.modifyManifestLockFee
import com.radixdlt.sargon.newTransactionManifestFromInstructionsStringAndBlobs
import com.radixdlt.sargon.transactionManifestToString

fun TransactionManifest.Companion.init(
    instructionsString: String,
    networkId: NetworkId,
    blobs: Blobs = Blobs.init()
) = newTransactionManifestFromInstructionsStringAndBlobs(
    instructionsString = instructionsString,
    networkId = networkId,
    blobs = blobs
)

fun TransactionManifest.Companion.createFungibleToken(
    addressOfOwner: AccountAddress
) = manifestCreateFungibleToken(addressOfOwner = addressOfOwner)

fun TransactionManifest.Companion.createNonFungibleToken(
    addressOfOwner: AccountAddress
) = manifestCreateNonFungibleToken(addressOfOwner = addressOfOwner)

fun TransactionManifest.Companion.createFungibleTokenWithMetadata(
    addressOfOwner: AccountAddress,
    initialSupply: Decimal192,
    metadata: TokenDefinitionMetadata
) = manifestCreateFungibleTokenWithMetadata(
    addressOfOwner = addressOfOwner,
    initialSupply = initialSupply,
    metadata = metadata
)

fun TransactionManifest.Companion.createMultipleFungibleTokens(
    addressOfOwner: AccountAddress
) = manifestCreateMultipleFungibleTokens(addressOfOwner = addressOfOwner)

fun TransactionManifest.Companion.createMultipleNonFungibleTokens(
    addressOfOwner: AccountAddress
) = manifestCreateMultipleNonFungibleTokens(addressOfOwner = addressOfOwner)

fun TransactionManifest.Companion.faucet(
    includeLockFeeInstruction: Boolean,
    addressOfReceivingAccount: AccountAddress
) = manifestForFaucet(
    includeLockFeeInstruction = includeLockFeeInstruction,
    addressOfReceivingAccount = addressOfReceivingAccount
)

fun TransactionManifest.Companion.markingAccountAsDAppDefinitionType(
    accountAddress: AccountAddress
) = manifestMarkingAccountAsDappDefinitionType(
    accountAddress = accountAddress
)

fun TransactionManifest.Companion.perAssetTransfers(
    transfers: PerAssetTransfers
) = manifestPerAssetTransfers(
    transfers = transfers
)

fun TransactionManifest.Companion.perRecipientTransfers(
    transfers: PerRecipientAssetTransfers
) = manifestPerRecipientTransfers(
    transfers = transfers
)

fun TransactionManifest.Companion.setOwnerKeysHashes(
    addressOfAccountOrPersona: AddressOfAccountOrPersona,
    ownerKeyHashes: List<PublicKeyHash>
) = manifestSetOwnerKeysHashes(
    addressOfAccountOrPersona = addressOfAccountOrPersona,
    ownerKeyHashes = ownerKeyHashes
)

fun TransactionManifest.Companion.stakesClaim(
    accountAddress: AccountAddress,
    stakeClaims: List<StakeClaim>
) = manifestStakesClaim(
    accountAddress = accountAddress,
    stakeClaims = stakeClaims
)

fun TransactionManifest.Companion.thirdPartyDepositUpdate(
    accountAddress: AccountAddress,
    from: ThirdPartyDeposits,
    to: ThirdPartyDeposits
) = manifestThirdPartyDepositUpdate(
    accountAddress = accountAddress,
    from = from,
    to = to
)

fun TransactionManifest.modifyAddGuarantees(
    guarantees: List<TransactionGuarantee>
) = modifyManifestAddGuarantees(manifest = this, guarantees = guarantees)

fun TransactionManifest.modifyLockFee(
    addressOfFeePayer: AccountAddress,
    fee: Decimal192?
) = modifyManifestLockFee(manifest = this, addressOfFeePayer = addressOfFeePayer, fee = fee)

val TransactionManifest.string: String
    get() = transactionManifestToString(manifest = this)