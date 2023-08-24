// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::utils::*;
use anyhow::Result;
use diem_types::on_chain_config::{FeatureFlag as DiemFeatureFlag, Features as DiemFeatures};
use move_model::{code_writer::CodeWriter, emit, emitln, model::Loc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, Debug)]
pub struct Features {
    #[serde(default)]
    pub enabled: Vec<FeatureFlag>,
    #[serde(default)]
    pub disabled: Vec<FeatureFlag>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[allow(non_camel_case_types)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    CodeDependencyCheck,
    CollectAndDistributeGasFees,
    TreatFriendAsPrivate,
    Sha512AndRipeMd160Natives,
    DiemStdChainIdNatives,
    VMBinaryFormatV6,
    MultiEd25519PkValidateV2Natives,
    Blake2b256Native,
    ResourceGroups,
    MultisigAccounts,
    DelegationPools,
    CryptographyAlgebraNatives,
    Bls12381Structures,
    Ed25519PubkeyValidateReturnFalseWrongLength,
    StructConstructors,
    PeriodicalRewardRateReduction,
    PartialGovernanceVoting,
    SignatureCheckerV2,
    StorageSlotMetadata,
    ChargeInvariantViolation,
    DelegationPoolPartialGovernanceVoting,
    GasPayerEnabled,
    DiemUniqueIdentifiers,
}

fn generate_features_blob(writer: &CodeWriter, data: &[u64]) {
    emitln!(writer, "vector[");
    writer.indent();
    for (i, b) in data.iter().enumerate() {
        if i % 20 == 0 {
            if i > 0 {
                emitln!(writer);
            }
        } else {
            emit!(writer, " ");
        }
        emit!(writer, "{},", b);
    }
    emitln!(writer);
    writer.unindent();
    emit!(writer, "]")
}

pub fn generate_feature_upgrade_proposal(
    features: &Features,
    is_testnet: bool,
    next_execution_hash: Vec<u8>,
) -> Result<Vec<(String, String)>> {
    let mut result = vec![];

    let enabled = features
        .enabled
        .iter()
        .map(|f| DiemFeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();
    let disabled = features
        .disabled
        .iter()
        .map(|f| DiemFeatureFlag::from(f.clone()) as u64)
        .collect::<Vec<_>>();

    assert!(enabled.len() < u16::MAX as usize);
    assert!(disabled.len() < u16::MAX as usize);

    let writer = CodeWriter::new(Loc::default());

    emitln!(writer, "// Modifying on-chain feature flags: ");
    emitln!(writer, "// Enabled Features: {:?}", features.enabled);
    emitln!(writer, "// Disabled Features: {:?}", features.disabled);
    emitln!(writer, "//");

    let proposal = generate_governance_proposal(
        &writer,
        is_testnet,
        next_execution_hash.clone(),
        &["std::features"],
        |writer| {
            emit!(writer, "let enabled_blob: vector<u64> = ");
            generate_features_blob(writer, &enabled);
            emitln!(writer, ";\n");

            emit!(writer, "let disabled_blob: vector<u64> = ");
            generate_features_blob(writer, &disabled);
            emitln!(writer, ";\n");

            if is_testnet && next_execution_hash.is_empty() {
                emitln!(
                    writer,
                    "features::change_feature_flags(framework_signer, enabled_blob, disabled_blob);"
                );
                emitln!(writer, "diem_governance::reconfigure(framework_signer);");
            } else {
                emitln!(
                    writer,
                    "features::change_feature_flags(&framework_signer, enabled_blob, disabled_blob);"
                );
                emitln!(writer, "diem_governance::reconfigure(&framework_signer);");
            }
        },
    );

    result.push(("features".to_string(), proposal));
    Ok(result)
}

impl From<FeatureFlag> for DiemFeatureFlag {
    fn from(f: FeatureFlag) -> Self {
        match f {
            FeatureFlag::CodeDependencyCheck => DiemFeatureFlag::CODE_DEPENDENCY_CHECK,
            FeatureFlag::CollectAndDistributeGasFees => {
                DiemFeatureFlag::COLLECT_AND_DISTRIBUTE_GAS_FEES
            },
            FeatureFlag::TreatFriendAsPrivate => DiemFeatureFlag::TREAT_FRIEND_AS_PRIVATE,
            FeatureFlag::Sha512AndRipeMd160Natives => {
                DiemFeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES
            },
            FeatureFlag::DiemStdChainIdNatives => DiemFeatureFlag::DIEM_STD_CHAIN_ID_NATIVES,
            FeatureFlag::VMBinaryFormatV6 => DiemFeatureFlag::VM_BINARY_FORMAT_V6,
            FeatureFlag::MultiEd25519PkValidateV2Natives => {
                DiemFeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES
            },
            FeatureFlag::Blake2b256Native => DiemFeatureFlag::BLAKE2B_256_NATIVE,
            FeatureFlag::ResourceGroups => DiemFeatureFlag::RESOURCE_GROUPS,
            FeatureFlag::MultisigAccounts => DiemFeatureFlag::MULTISIG_ACCOUNTS,
            FeatureFlag::DelegationPools => DiemFeatureFlag::DELEGATION_POOLS,
            FeatureFlag::CryptographyAlgebraNatives => {
                DiemFeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES
            },
            FeatureFlag::Bls12381Structures => DiemFeatureFlag::BLS12_381_STRUCTURES,
            FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength => {
                DiemFeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH
            },
            FeatureFlag::StructConstructors => DiemFeatureFlag::STRUCT_CONSTRUCTORS,
            FeatureFlag::PeriodicalRewardRateReduction => {
                DiemFeatureFlag::PERIODICAL_REWARD_RATE_DECREASE
            },
            FeatureFlag::PartialGovernanceVoting => DiemFeatureFlag::PARTIAL_GOVERNANCE_VOTING,
            FeatureFlag::SignatureCheckerV2 => DiemFeatureFlag::SIGNATURE_CHECKER_V2,
            FeatureFlag::StorageSlotMetadata => DiemFeatureFlag::STORAGE_SLOT_METADATA,
            FeatureFlag::ChargeInvariantViolation => DiemFeatureFlag::CHARGE_INVARIANT_VIOLATION,
            FeatureFlag::DelegationPoolPartialGovernanceVoting => {
                DiemFeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING
            },
            FeatureFlag::GasPayerEnabled => DiemFeatureFlag::GAS_PAYER_ENABLED,
            FeatureFlag::DiemUniqueIdentifiers => DiemFeatureFlag::DIEM_UNIQUE_IDENTIFIERS,
        }
    }
}

// We don't need this implementation. Just to make sure we have an exhaustive 1-1 mapping between the two structs.
impl From<DiemFeatureFlag> for FeatureFlag {
    fn from(f: DiemFeatureFlag) -> Self {
        match f {
            DiemFeatureFlag::CODE_DEPENDENCY_CHECK => FeatureFlag::CodeDependencyCheck,
            DiemFeatureFlag::COLLECT_AND_DISTRIBUTE_GAS_FEES => {
                FeatureFlag::CollectAndDistributeGasFees
            },
            DiemFeatureFlag::TREAT_FRIEND_AS_PRIVATE => FeatureFlag::TreatFriendAsPrivate,
            DiemFeatureFlag::SHA_512_AND_RIPEMD_160_NATIVES => {
                FeatureFlag::Sha512AndRipeMd160Natives
            },
            DiemFeatureFlag::DIEM_STD_CHAIN_ID_NATIVES => FeatureFlag::DiemStdChainIdNatives,
            DiemFeatureFlag::VM_BINARY_FORMAT_V6 => FeatureFlag::VMBinaryFormatV6,
            DiemFeatureFlag::MULTI_ED25519_PK_VALIDATE_V2_NATIVES => {
                FeatureFlag::MultiEd25519PkValidateV2Natives
            },
            DiemFeatureFlag::BLAKE2B_256_NATIVE => FeatureFlag::Blake2b256Native,
            DiemFeatureFlag::RESOURCE_GROUPS => FeatureFlag::ResourceGroups,
            DiemFeatureFlag::MULTISIG_ACCOUNTS => FeatureFlag::MultisigAccounts,
            DiemFeatureFlag::DELEGATION_POOLS => FeatureFlag::DelegationPools,
            DiemFeatureFlag::CRYPTOGRAPHY_ALGEBRA_NATIVES => {
                FeatureFlag::CryptographyAlgebraNatives
            },
            DiemFeatureFlag::BLS12_381_STRUCTURES => FeatureFlag::Bls12381Structures,
            DiemFeatureFlag::ED25519_PUBKEY_VALIDATE_RETURN_FALSE_WRONG_LENGTH => {
                FeatureFlag::Ed25519PubkeyValidateReturnFalseWrongLength
            },
            DiemFeatureFlag::STRUCT_CONSTRUCTORS => FeatureFlag::StructConstructors,
            DiemFeatureFlag::PERIODICAL_REWARD_RATE_DECREASE => {
                FeatureFlag::PeriodicalRewardRateReduction
            },
            DiemFeatureFlag::PARTIAL_GOVERNANCE_VOTING => FeatureFlag::PartialGovernanceVoting,
            DiemFeatureFlag::SIGNATURE_CHECKER_V2 => FeatureFlag::SignatureCheckerV2,
            DiemFeatureFlag::STORAGE_SLOT_METADATA => FeatureFlag::StorageSlotMetadata,
            DiemFeatureFlag::CHARGE_INVARIANT_VIOLATION => FeatureFlag::ChargeInvariantViolation,
            DiemFeatureFlag::DELEGATION_POOL_PARTIAL_GOVERNANCE_VOTING => {
                FeatureFlag::DelegationPoolPartialGovernanceVoting
            },
            DiemFeatureFlag::GAS_PAYER_ENABLED => FeatureFlag::GasPayerEnabled,
            DiemFeatureFlag::DIEM_UNIQUE_IDENTIFIERS => FeatureFlag::DiemUniqueIdentifiers,
        }
    }
}

impl Features {
    // Compare if the current feature set is different from features that has been enabled on chain.
    pub(crate) fn has_modified(&self, on_chain_features: &DiemFeatures) -> bool {
        self.enabled
            .iter()
            .any(|f| !on_chain_features.is_enabled(DiemFeatureFlag::from(f.clone())))
            || self
                .disabled
                .iter()
                .any(|f| on_chain_features.is_enabled(DiemFeatureFlag::from(f.clone())))
    }
}
