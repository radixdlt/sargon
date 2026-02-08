## External Account Signing - Specification

This document describes the intended behavior for signing when a transaction or subintent
requires authorization from accounts that are not present in the local profile.

### Goals

- Support multi-party/multi-sig accounts where the user only controls a subset of the
  required factors.
- Collect valid signatures for the factors the user can provide, without requiring full
  reconstruction of the account's security structure.
- Keep the behavior consistent even if internal implementation details change.

### Definitions

- **External account**: An account required to sign but not present in the local profile.
- **Access rule**: On-ledger rule describing which proofs are acceptable for signing.
- **Required NFTs**: The set of non-fungible IDs referenced by the access rule.
- **Factor instance lookup**: A mapping from required NFT IDs to the user's available factor
  instances.

### High-level Behavior

1) **Detect external accounts**
   - Inspect the transaction/subintent to determine which accounts require signatures.
   - Any account not present in the local profile is treated as external.

2) **Resolve access rules**
   - For each external account, retrieve the access rule configuration from on-ledger data.
   - The access rule can be sourced from either the account entity or its access controller.
   - The specific storage location may evolve; the system should treat this as a retrieval step.

3) **Extract required NFT IDs**
   - Parse the access rule and extract all referenced non-fungible global IDs.
   - Only the IDs are required for the next step; the full rule structure is not needed.

4) **Lookup available factor instances**
   - Given the set of required NFT IDs, resolve the user's available factor instances.
   - This lookup can be stubbed or delegated to a dedicated component.

5) **Request signatures**
   - Ask the user to sign with each resolved factor instance.
   - Do not attempt to satisfy the full access rule or enforce thresholds.
   - Any successfully produced signature is accepted.

6) **Merge signatures**
   - Combine external signatures with any signatures obtained from profile-based signing.
   - Construct the final signed intent/subintent with the union of signatures.

### Non-goals

- Reconstructing full security structures for external accounts.
- Determining whether the collected signatures fully satisfy the access rule.
- Rejecting partial signatures solely because they are insufficient for final submission.

### Expected Outcomes

- If the user controls at least one required factor instance, they can contribute a valid
  signature for the external account.
- The system can assemble a signed intent/subintent from partial signatures.
- Final validation (if all required signatures are present) is handled at submission time
  or by the receiving system.
