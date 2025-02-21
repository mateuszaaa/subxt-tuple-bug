#[allow(non_snake_case)]
#[allow(dead_code, unused_imports, non_camel_case_types, unreachable_patterns)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 6usize] = [
        "System",
        "Timestamp",
        "Balances",
        "Sudo",
        "TransactionPayment",
        "Template",
    ];
    pub static RUNTIME_APIS: [&str; 9usize] = [
        "Core",
        "Metadata",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "SessionKeys",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "GenesisBuilder",
    ];
    #[doc = r" The error type that is returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::minimal_template_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::minimal_template_runtime::RuntimeCall;
    #[doc = r" The outer error enum represents the DispatchError's Module variant."]
    pub type Error = runtime_types::minimal_template_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn core(&self) -> core::Core {
                core::Core
            }
            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }
            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }
            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }
            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }
            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }
            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }
            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }
            pub fn genesis_builder(&self) -> genesis_builder::GenesisBuilder {
                genesis_builder::GenesisBuilder
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Core` runtime api that every Substrate runtime needs to implement."]
            pub struct Core;
            impl Core {
                #[doc = " Returns the version of the runtime."]
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Version,
                    types::version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }
                #[doc = " Execute the given block."]
                pub fn execute_block(
                    &self,
                    block: types::execute_block::Block,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ExecuteBlock,
                    types::execute_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock { block },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }
                #[doc = " Initialize a block with the given header and return the runtime executive mode."]
                pub fn initialize_block(
                    &self,
                    header: types::initialize_block::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InitializeBlock,
                    types::initialize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock { header },
                        [
                            132u8, 169u8, 113u8, 112u8, 80u8, 139u8, 113u8, 35u8, 41u8, 81u8, 36u8,
                            35u8, 37u8, 202u8, 29u8, 207u8, 205u8, 229u8, 145u8, 7u8, 133u8, 94u8,
                            25u8, 108u8, 233u8, 86u8, 234u8, 29u8, 236u8, 57u8, 56u8, 186u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod version {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_version::RuntimeVersion;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Version {}
                pub mod execute_block {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExecuteBlock {
                    pub block: execute_block::Block,
                }
                pub mod initialize_block {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::ExtrinsicInclusionMode;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InitializeBlock {
                    pub header: initialize_block::Header,
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `Metadata` api trait that returns metadata for the runtime."]
            pub struct Metadata;
            impl Metadata {
                #[doc = " Returns the metadata of a runtime."]
                pub fn metadata(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::Metadata,
                    types::metadata::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }
                #[doc = " Returns the metadata at a given version."]
                #[doc = ""]
                #[doc = " If the given `version` isn't supported, this will return `None`."]
                #[doc = " Use [`Self::metadata_versions`] to find out about supported metadata version of the runtime."]
                pub fn metadata_at_version(
                    &self,
                    version: types::metadata_at_version::Version,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataAtVersion,
                    types::metadata_at_version::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion { version },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }
                #[doc = " Returns the supported metadata versions."]
                #[doc = ""]
                #[doc = " This can be used to call `metadata_at_version`."]
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::MetadataVersions,
                    types::metadata_versions::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod metadata {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_core::OpaqueMetadata;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Metadata {}
                pub mod metadata_at_version {
                    use super::runtime_types;
                    pub type Version = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataAtVersion {
                    pub version: metadata_at_version::Version,
                }
                pub mod metadata_versions {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `BlockBuilder` api trait that provides the required functionality for building a block."]
            pub struct BlockBuilder;
            impl BlockBuilder {
                #[doc = " Apply the given extrinsic."]
                #[doc = ""]
                #[doc = " Returns an inclusion outcome which specifies if this extrinsic is included in"]
                #[doc = " this block or not."]
                pub fn apply_extrinsic(
                    &self,
                    extrinsic: types::apply_extrinsic::Extrinsic,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ApplyExtrinsic,
                    types::apply_extrinsic::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic { extrinsic },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }
                #[doc = " Finish the current block."]
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::FinalizeBlock,
                    types::finalize_block::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }
                #[doc = " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
                pub fn inherent_extrinsics(
                    &self,
                    inherent: types::inherent_extrinsics::Inherent,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::InherentExtrinsics,
                    types::inherent_extrinsics::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics { inherent },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }
                #[doc = " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                pub fn check_inherents(
                    &self,
                    block: types::check_inherents::Block,
                    data: types::check_inherents::Data,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::CheckInherents,
                    types::check_inherents::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents { block, data },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod apply_extrinsic {
                    use super::runtime_types;
                    pub type Extrinsic = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ApplyExtrinsic {
                    pub extrinsic: apply_extrinsic::Extrinsic,
                }
                pub mod finalize_block {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FinalizeBlock {}
                pub mod inherent_extrinsics {
                    use super::runtime_types;
                    pub type Inherent = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: subxt :: ext :: subxt_core :: alloc :: vec :: Vec < :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InherentExtrinsics {
                    pub inherent: inherent_extrinsics::Inherent,
                }
                pub mod check_inherents {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub type Data = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct CheckInherents {
                    pub block: check_inherents::Block,
                    pub data: check_inherents::Data,
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The `TaggedTransactionQueue` api trait for interfering with the transaction queue."]
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                #[doc = " Validate the transaction."]
                #[doc = ""]
                #[doc = " This method is invoked by the transaction pool to learn details about given transaction."]
                #[doc = " The implementation should make sure to verify the correctness of the transaction"]
                #[doc = " against current state. The given `block_hash` corresponds to the hash of the block"]
                #[doc = " that is used as current state."]
                #[doc = ""]
                #[doc = " Note that this call may be performed by the pool multiple times and transactions"]
                #[doc = " might be verified in any possible order."]
                pub fn validate_transaction(
                    &self,
                    source: types::validate_transaction::Source,
                    tx: types::validate_transaction::Tx,
                    block_hash: types::validate_transaction::BlockHash,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::ValidateTransaction,
                    types::validate_transaction::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod validate_transaction {
                    use super::runtime_types;
                    pub type Source =
                        runtime_types::sp_runtime::transaction_validity::TransactionSource;
                    pub type Tx = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < runtime_types :: sp_runtime :: transaction_validity :: ValidTransaction , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidateTransaction {
                    pub source: validate_transaction::Source,
                    pub tx: validate_transaction::Tx,
                    pub block_hash: validate_transaction::BlockHash,
                }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The offchain worker api."]
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                #[doc = " Starts the off-chain task for given block header."]
                pub fn offchain_worker(
                    &self,
                    header: types::offchain_worker::Header,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::OffchainWorker,
                    types::offchain_worker::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker { header },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod offchain_worker {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct OffchainWorker {
                    pub header: offchain_worker::Header,
                }
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " Session keys runtime api."]
            pub struct SessionKeys;
            impl SessionKeys {
                #[doc = " Generate a set of session keys with optionally using the given seed."]
                #[doc = " The keys should be stored within the keystore exposed via runtime"]
                #[doc = " externalities."]
                #[doc = ""]
                #[doc = " The seed needs to be a valid `utf8` string."]
                #[doc = ""]
                #[doc = " Returns the concatenated SCALE encoded public keys."]
                pub fn generate_session_keys(
                    &self,
                    seed: types::generate_session_keys::Seed,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GenerateSessionKeys,
                    types::generate_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys { seed },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }
                #[doc = " Decode the given public session keys."]
                #[doc = ""]
                #[doc = " Returns the list of public raw public keys + key type."]
                pub fn decode_session_keys(
                    &self,
                    encoded: types::decode_session_keys::Encoded,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::DecodeSessionKeys,
                    types::decode_session_keys::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys { encoded },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod generate_session_keys {
                    use super::runtime_types;
                    pub type Seed = ::core::option::Option<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GenerateSessionKeys {
                    pub seed: generate_session_keys::Seed,
                }
                pub mod decode_session_keys {
                    use super::runtime_types;
                    pub type Encoded =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<(
                                ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                                runtime_types::sp_core::crypto::KeyTypeId,
                            )>,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DecodeSessionKeys {
                    pub encoded: decode_session_keys::Encoded,
                }
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " The API to query account nonce."]
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                #[doc = " Get current account nonce of given `AccountId`."]
                pub fn account_nonce(
                    &self,
                    account: types::account_nonce::Account,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::AccountNonce,
                    types::account_nonce::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce { account },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod account_nonce {
                    use super::runtime_types;
                    pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u32;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountNonce {
                    pub account: account_nonce::Account,
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt: types::query_info::Uxt,
                    len: types::query_info::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryInfo,
                    types::query_info::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        types::QueryInfo { uxt, len },
                        [
                            24u8, 42u8, 137u8, 188u8, 70u8, 224u8, 145u8, 166u8, 22u8, 122u8, 72u8,
                            73u8, 139u8, 92u8, 235u8, 144u8, 95u8, 193u8, 39u8, 238u8, 220u8,
                            222u8, 144u8, 83u8, 226u8, 13u8, 83u8, 120u8, 142u8, 111u8, 231u8,
                            69u8,
                        ],
                    )
                }
                pub fn query_fee_details(
                    &self,
                    uxt: types::query_fee_details::Uxt,
                    len: types::query_fee_details::Len,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryFeeDetails,
                    types::query_fee_details::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        types::QueryFeeDetails { uxt, len },
                        [
                            182u8, 198u8, 165u8, 34u8, 109u8, 68u8, 119u8, 118u8, 116u8, 240u8,
                            160u8, 4u8, 1u8, 14u8, 169u8, 207u8, 142u8, 193u8, 16u8, 89u8, 179u8,
                            215u8, 246u8, 166u8, 194u8, 49u8, 110u8, 19u8, 144u8, 241u8, 161u8,
                            190u8,
                        ],
                    )
                }
                pub fn query_weight_to_fee(
                    &self,
                    weight: types::query_weight_to_fee::Weight,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryWeightToFee,
                    types::query_weight_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee { weight },
                        [
                            43u8, 38u8, 7u8, 127u8, 93u8, 202u8, 221u8, 100u8, 79u8, 26u8, 61u8,
                            51u8, 106u8, 250u8, 233u8, 161u8, 32u8, 72u8, 198u8, 28u8, 94u8, 150u8,
                            8u8, 143u8, 58u8, 148u8, 69u8, 198u8, 215u8, 27u8, 186u8, 242u8,
                        ],
                    )
                }
                pub fn query_length_to_fee(
                    &self,
                    length: types::query_length_to_fee::Length,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::QueryLengthToFee,
                    types::query_length_to_fee::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee { length },
                        [
                            129u8, 136u8, 216u8, 144u8, 66u8, 102u8, 146u8, 169u8, 202u8, 11u8,
                            0u8, 146u8, 200u8, 172u8, 84u8, 238u8, 129u8, 32u8, 214u8, 169u8,
                            130u8, 111u8, 112u8, 68u8, 51u8, 8u8, 174u8, 90u8, 81u8, 46u8, 0u8,
                            249u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod query_info {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                                ::core::primitive::u64,
                                runtime_types::sp_weights::weight_v2::Weight,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryInfo {
                    pub uxt: query_info::Uxt,
                    pub len: query_info::Len,
                }
                pub mod query_fee_details {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: ext :: subxt_core :: utils :: UncheckedExtrinsic < :: subxt :: ext :: subxt_core :: utils :: MultiAddress < :: subxt :: ext :: subxt_core :: utils :: AccountId32 , () > , runtime_types :: minimal_template_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::FeeDetails<
                                ::core::primitive::u64,
                            >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryFeeDetails {
                    pub uxt: query_fee_details::Uxt,
                    pub len: query_fee_details::Len,
                }
                pub mod query_weight_to_fee {
                    use super::runtime_types;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u64;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryWeightToFee {
                    pub weight: query_weight_to_fee::Weight,
                }
                pub mod query_length_to_fee {
                    use super::runtime_types;
                    pub type Length = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u64;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct QueryLengthToFee {
                    pub length: query_length_to_fee::Length,
                }
            }
        }
        pub mod genesis_builder {
            use super::root_mod;
            use super::runtime_types;
            #[doc = " API to interact with RuntimeGenesisConfig for the runtime"]
            pub struct GenesisBuilder;
            impl GenesisBuilder {
                #[doc = " Build `RuntimeGenesisConfig` from a JSON blob not using any defaults and store it in the"]
                #[doc = " storage."]
                #[doc = ""]
                #[doc = " In the case of a FRAME-based runtime, this function deserializes the full `RuntimeGenesisConfig` from the given JSON blob and"]
                #[doc = " puts it into the storage. If the provided JSON blob is incorrect or incomplete or the"]
                #[doc = " deserialization fails, an error is returned."]
                #[doc = ""]
                #[doc = " Please note that provided JSON blob must contain all `RuntimeGenesisConfig` fields, no"]
                #[doc = " defaults will be used."]
                pub fn build_state(
                    &self,
                    json: types::build_state::Json,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::BuildState,
                    types::build_state::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "build_state",
                        types::BuildState { json },
                        [
                            203u8, 233u8, 104u8, 116u8, 111u8, 131u8, 201u8, 235u8, 117u8, 116u8,
                            140u8, 185u8, 93u8, 25u8, 155u8, 210u8, 56u8, 49u8, 23u8, 32u8, 253u8,
                            92u8, 149u8, 241u8, 85u8, 245u8, 137u8, 45u8, 209u8, 189u8, 81u8, 2u8,
                        ],
                    )
                }
                #[doc = " Returns a JSON blob representation of the built-in `RuntimeGenesisConfig` identified by"]
                #[doc = " `id`."]
                #[doc = ""]
                #[doc = " If `id` is `None` the function returns JSON blob representation of the default"]
                #[doc = " `RuntimeGenesisConfig` struct of the runtime. Implementation must provide default"]
                #[doc = " `RuntimeGenesisConfig`."]
                #[doc = ""]
                #[doc = " Otherwise function returns a JSON representation of the built-in, named"]
                #[doc = " `RuntimeGenesisConfig` preset identified by `id`, or `None` if such preset does not"]
                #[doc = " exist. Returned `Vec<u8>` contains bytes of JSON blob (patch) which comprises a list of"]
                #[doc = " (potentially nested) key-value pairs that are intended for customizing the default"]
                #[doc = " runtime genesis config. The patch shall be merged (rfc7386) with the JSON representation"]
                #[doc = " of the default `RuntimeGenesisConfig` to create a comprehensive genesis config that can"]
                #[doc = " be used in `build_state` method."]
                pub fn get_preset(
                    &self,
                    id: types::get_preset::Id,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::GetPreset,
                    types::get_preset::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "get_preset",
                        types::GetPreset { id },
                        [
                            43u8, 153u8, 23u8, 52u8, 113u8, 161u8, 227u8, 122u8, 169u8, 135u8,
                            119u8, 8u8, 128u8, 33u8, 143u8, 235u8, 13u8, 173u8, 58u8, 121u8, 178u8,
                            223u8, 66u8, 217u8, 22u8, 244u8, 168u8, 113u8, 202u8, 186u8, 241u8,
                            124u8,
                        ],
                    )
                }
                #[doc = " Returns a list of identifiers for available builtin `RuntimeGenesisConfig` presets."]
                #[doc = ""]
                #[doc = " The presets from the list can be queried with [`GenesisBuilder::get_preset`] method. If"]
                #[doc = " no named presets are provided by the runtime the list is empty."]
                pub fn preset_names(
                    &self,
                ) -> ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload<
                    types::PresetNames,
                    types::preset_names::output::Output,
                > {
                    ::subxt::ext::subxt_core::runtime_api::payload::StaticPayload::new_static(
                        "GenesisBuilder",
                        "preset_names",
                        types::PresetNames {},
                        [
                            150u8, 117u8, 54u8, 129u8, 221u8, 130u8, 186u8, 71u8, 13u8, 140u8,
                            77u8, 180u8, 141u8, 37u8, 22u8, 219u8, 149u8, 218u8, 186u8, 206u8,
                            80u8, 42u8, 165u8, 41u8, 99u8, 184u8, 73u8, 37u8, 125u8, 188u8, 167u8,
                            122u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod build_state {
                    use super::runtime_types;
                    pub type Json =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            (),
                            ::subxt::ext::subxt_core::alloc::string::String,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BuildState {
                    pub json: build_state::Json,
                }
                pub mod get_preset {
                    use super::runtime_types;
                    pub type Id =
                        ::core::option::Option<::subxt::ext::subxt_core::alloc::string::String>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct GetPreset {
                    pub id: get_preset::Id,
                }
                pub mod preset_names {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::string::String,
                        >;
                    }
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PresetNames {}
            }
        }
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn template(&self) -> template::storage::StorageApi {
            template::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt::ext::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                155u8, 254u8, 219u8, 47u8, 66u8, 216u8, 109u8, 142u8, 217u8, 96u8, 70u8, 3u8,
                189u8, 172u8, 145u8, 238u8, 68u8, 52u8, 224u8, 152u8, 1u8, 34u8, 51u8, 97u8, 166u8,
                36u8, 64u8, 67u8, 187u8, 86u8, 81u8, 107u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code."]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set some items of storage."]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill some items from storage."]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Make some on-chain remark and emit event."]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DoTask {
                    pub task: do_task::Task,
                }
                pub mod do_task {
                    use super::runtime_types;
                    pub type Task = runtime_types::minimal_template_runtime::RuntimeTask;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for DoTask {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "do_task";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgrade {
                    pub code_hash: authorize_upgrade::CodeHash,
                }
                pub mod authorize_upgrade {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub struct AuthorizeUpgradeWithoutChecks {
                    pub code_hash: authorize_upgrade_without_checks::CodeHash,
                }
                pub mod authorize_upgrade_without_checks {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub struct ApplyAuthorizedUpgrade {
                    pub code: apply_authorized_upgrade::Code,
                }
                pub mod apply_authorized_upgrade {
                    use super::runtime_types;
                    pub type Code =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "apply_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "Can be executed by every `origin`."]
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Remark>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetHeapPages>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCode>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                #[doc = "version!"]
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillStorage>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::KillPrefix>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
                pub fn do_task(
                    &self,
                    task: types::do_task::Task,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::DoTask>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "do_task",
                        types::DoTask { task },
                        [
                            199u8, 103u8, 231u8, 124u8, 182u8, 26u8, 42u8, 190u8, 140u8, 155u8,
                            96u8, 212u8, 167u8, 213u8, 125u8, 28u8, 209u8, 70u8, 241u8, 190u8,
                            219u8, 72u8, 151u8, 74u8, 16u8, 245u8, 77u8, 29u8, 3u8, 20u8, 65u8,
                            158u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade(
                    &self,
                    code_hash: types::authorize_upgrade::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade { code_hash },
                        [
                            4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                            254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                            58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                            172u8,
                        ],
                    )
                }
                #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                #[doc = "later."]
                #[doc = ""]
                #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                #[doc = ""]
                #[doc = "This call requires Root origin."]
                pub fn authorize_upgrade_without_checks(
                    &self,
                    code_hash: types::authorize_upgrade_without_checks::CodeHash,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::AuthorizeUpgradeWithoutChecks,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade_without_checks",
                        types::AuthorizeUpgradeWithoutChecks { code_hash },
                        [
                            126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                            136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                            197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                            233u8,
                        ],
                    )
                }
                #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                #[doc = ""]
                #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                #[doc = "remains unchanged and that the spec version has increased."]
                #[doc = ""]
                #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                #[doc = ""]
                #[doc = "All origins are allowed."]
                pub fn apply_authorized_upgrade(
                    &self,
                    code: types::apply_authorized_upgrade::Code,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ApplyAuthorizedUpgrade,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "apply_authorized_upgrade",
                        types::ApplyAuthorizedUpgrade { code },
                        [
                            232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
                            156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
                            32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::ext::subxt_core::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Hash = ::subxt::ext::subxt_core::utils::H256;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] has started executing"]
            pub struct TaskStarted {
                pub task: task_started::Task,
            }
            pub mod task_started {
                use super::runtime_types;
                pub type Task = runtime_types::minimal_template_runtime::RuntimeTask;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskStarted {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskStarted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] has finished executing."]
            pub struct TaskCompleted {
                pub task: task_completed::Task,
            }
            pub mod task_completed {
                use super::runtime_types;
                pub type Task = runtime_types::minimal_template_runtime::RuntimeTask;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskCompleted {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskCompleted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [`Task`] failed during execution."]
            pub struct TaskFailed {
                pub task: task_failed::Task,
                pub err: task_failed::Err,
            }
            pub mod task_failed {
                use super::runtime_types;
                pub type Task = runtime_types::minimal_template_runtime::RuntimeTask;
                pub type Err = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TaskFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "TaskFailed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An upgrade was authorized."]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt::ext::subxt_core::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u64>,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod inherents_applied {
                    use super::runtime_types;
                    pub type InherentsApplied = ::core::primitive::bool;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::ext::subxt_core::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData =
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::minimal_template_runtime::RuntimeEvent,
                            ::subxt::ext::subxt_core::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics = ::subxt::ext::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
                pub mod authorized_upgrade {
                    use super::runtime_types;
                    pub type AuthorizedUpgrade =
                        runtime_types::frame_system::CodeUpgradeAuthorization;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        (),
                        [
                            134u8, 36u8, 200u8, 97u8, 250u8, 93u8, 194u8, 130u8, 234u8, 169u8,
                            160u8, 176u8, 58u8, 142u8, 63u8, 133u8, 29u8, 94u8, 102u8, 192u8, 82u8,
                            6u8, 105u8, 248u8, 238u8, 246u8, 26u8, 107u8, 39u8, 219u8, 103u8,
                            212u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            134u8, 36u8, 200u8, 97u8, 250u8, 93u8, 194u8, 130u8, 234u8, 169u8,
                            160u8, 176u8, 58u8, 142u8, 63u8, 133u8, 29u8, 94u8, 102u8, 192u8, 82u8,
                            6u8, 105u8, 248u8, 238u8, 246u8, 26u8, 107u8, 39u8, 219u8, 103u8,
                            212u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        (),
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " Whether all inherents have been applied."]
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inherents_applied::InherentsApplied,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        (),
                        [
                            132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
                            223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
                            109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_weight::BlockWeight,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        (),
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        (),
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        (),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::block_hash::Param0,
                    >,
                    types::block_hash::BlockHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        (),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::extrinsic_data::Param0,
                    >,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::number::Number,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Number",
                        (),
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::parent_hash::ParentHash,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        (),
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::digest::Digest,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Digest",
                        (),
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::events::Events,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Events",
                        (),
                        [
                            69u8, 9u8, 5u8, 10u8, 14u8, 215u8, 18u8, 239u8, 238u8, 100u8, 67u8,
                            153u8, 9u8, 5u8, 245u8, 16u8, 205u8, 146u8, 11u8, 181u8, 162u8, 47u8,
                            196u8, 59u8, 131u8, 220u8, 60u8, 87u8, 248u8, 63u8, 139u8, 177u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_count::EventCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        (),
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        (),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::event_topics::Param0,
                    >,
                    types::event_topics::EventTopics,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        (),
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        (),
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        (),
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::execution_phase::ExecutionPhase,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        (),
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
                #[doc = " `Some` if a code upgrade has been authorized."]
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorized_upgrade::AuthorizedUpgrade,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        (),
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_weights::RuntimeDbWeight,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                #[doc = " Get the chain's in-code version."]
                pub fn version(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u16,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "[`Config::MinimumPeriod`]."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _None_."]
                #[doc = ""]
                #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                #[doc = "block to execute any other calls."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub fn set(
                    &self,
                    now: types::set::Now,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Set>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::now::Now,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        (),
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Whether the timestamp has been updated in this block."]
                #[doc = ""]
                #[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
                #[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
                pub fn did_update(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::did_update::DidUpdate,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        (),
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks."]
                #[doc = ""]
                #[doc = " Be aware that this is different to the *expected* period that the block production"]
                #[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
                #[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
                #[doc = " period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Error` enum of this pallet."]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Value = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Amount = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibility of churn)."]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type NewFree = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub struct ForceAdjustTotalIssuance {
                    pub direction: force_adjust_total_issuance::Direction,
                    #[codec(compact)]
                    pub delta: force_adjust_total_issuance::Delta,
                }
                pub mod force_adjust_total_issuance {
                    use super::runtime_types;
                    pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                    pub type Delta = ::core::primitive::u64;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_adjust_total_issuance";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Burn the specified liquid free balance from the origin account."]
                #[doc = ""]
                #[doc = "If the origin's account ends up below the existential deposit as a result"]
                #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                #[doc = ""]
                #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                pub struct Burn {
                    #[codec(compact)]
                    pub value: burn::Value,
                    pub keep_alive: burn::KeepAlive,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u64;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Burn {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "burn";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            55u8, 189u8, 102u8, 71u8, 231u8, 179u8, 169u8, 229u8, 206u8, 42u8,
                            10u8, 117u8, 36u8, 34u8, 30u8, 145u8, 227u8, 105u8, 252u8, 188u8,
                            162u8, 0u8, 84u8, 24u8, 123u8, 241u8, 152u8, 118u8, 181u8, 182u8,
                            137u8, 241u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            15u8, 9u8, 232u8, 170u8, 117u8, 161u8, 184u8, 255u8, 3u8, 155u8, 165u8,
                            226u8, 151u8, 189u8, 226u8, 21u8, 14u8, 70u8, 17u8, 43u8, 181u8, 59u8,
                            158u8, 222u8, 167u8, 59u8, 61u8, 233u8, 78u8, 108u8, 161u8, 192u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            126u8, 208u8, 16u8, 244u8, 94u8, 74u8, 166u8, 13u8, 149u8, 95u8, 88u8,
                            250u8, 95u8, 0u8, 43u8, 57u8, 18u8, 129u8, 24u8, 210u8, 91u8, 50u8,
                            251u8, 73u8, 126u8, 218u8, 244u8, 52u8, 36u8, 175u8, 28u8, 210u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::TransferAll>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            194u8, 160u8, 176u8, 186u8, 10u8, 132u8, 201u8, 232u8, 127u8, 9u8,
                            234u8, 37u8, 167u8, 77u8, 157u8, 228u8, 62u8, 207u8, 172u8, 240u8, 1u8,
                            10u8, 136u8, 97u8, 115u8, 13u8, 2u8, 61u8, 200u8, 235u8, 39u8, 97u8,
                        ],
                    )
                }
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibility of churn)."]
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            172u8, 128u8, 27u8, 158u8, 43u8, 74u8, 29u8, 195u8, 16u8, 214u8, 1u8,
                            128u8, 138u8, 144u8, 6u8, 236u8, 61u8, 41u8, 155u8, 208u8, 57u8, 42u8,
                            237u8, 229u8, 130u8, 214u8, 8u8, 93u8, 57u8, 219u8, 178u8, 197u8,
                        ],
                    )
                }
                #[doc = "Adjust the total issuance in a saturating way."]
                #[doc = ""]
                #[doc = "Can only be called by root and always needs a positive `delta`."]
                #[doc = ""]
                #[doc = "# Example"]
                pub fn force_adjust_total_issuance(
                    &self,
                    direction: types::force_adjust_total_issuance::Direction,
                    delta: types::force_adjust_total_issuance::Delta,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<
                    types::ForceAdjustTotalIssuance,
                > {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_adjust_total_issuance",
                        types::ForceAdjustTotalIssuance { direction, delta },
                        [
                            70u8, 239u8, 219u8, 254u8, 5u8, 227u8, 119u8, 182u8, 85u8, 96u8, 88u8,
                            71u8, 89u8, 73u8, 199u8, 50u8, 89u8, 157u8, 30u8, 121u8, 34u8, 184u8,
                            9u8, 118u8, 51u8, 182u8, 121u8, 109u8, 22u8, 155u8, 83u8, 198u8,
                        ],
                    )
                }
                #[doc = "Burn the specified liquid free balance from the origin account."]
                #[doc = ""]
                #[doc = "If the origin's account ends up below the existential deposit as a result"]
                #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                #[doc = ""]
                #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                pub fn burn(
                    &self,
                    value: types::burn::Value,
                    keep_alive: types::burn::KeepAlive,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Burn>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "burn",
                        types::Burn { value, keep_alive },
                        [
                            47u8, 248u8, 43u8, 36u8, 119u8, 205u8, 108u8, 21u8, 199u8, 180u8,
                            233u8, 162u8, 61u8, 240u8, 233u8, 246u8, 119u8, 19u8, 172u8, 171u8,
                            38u8, 155u8, 189u8, 243u8, 40u8, 235u8, 23u8, 221u8, 158u8, 47u8,
                            183u8, 10u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Free = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type To = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The `TotalIssuance` was forcefully changed."]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u64;
                pub type New = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TotalIssuanceForced {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "TotalIssuanceForced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u64;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u64;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u64>;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u64,
                            >,
                        >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            (),
                            ::core::primitive::u64,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::minimal_template_runtime::RuntimeHoldReason,
                            ::core::primitive::u64,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            (),
                            ::core::primitive::u64,
                        >,
                    >;
                    pub type Param0 = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_issuance::TotalIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        (),
                        [
                            157u8, 166u8, 49u8, 95u8, 245u8, 38u8, 96u8, 83u8, 23u8, 203u8, 246u8,
                            208u8, 251u8, 175u8, 40u8, 92u8, 163u8, 112u8, 43u8, 190u8, 14u8,
                            128u8, 45u8, 22u8, 157u8, 205u8, 181u8, 84u8, 79u8, 132u8, 160u8,
                            149u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        (),
                        [
                            33u8, 251u8, 197u8, 148u8, 61u8, 202u8, 85u8, 164u8, 169u8, 36u8,
                            163u8, 147u8, 243u8, 250u8, 178u8, 40u8, 199u8, 147u8, 196u8, 206u8,
                            163u8, 246u8, 154u8, 51u8, 208u8, 250u8, 127u8, 89u8, 118u8, 21u8,
                            225u8, 248u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        (),
                        [
                            0u8, 34u8, 91u8, 238u8, 87u8, 151u8, 107u8, 4u8, 104u8, 111u8, 47u8,
                            9u8, 10u8, 169u8, 173u8, 69u8, 182u8, 183u8, 227u8, 37u8, 148u8, 135u8,
                            184u8, 83u8, 67u8, 66u8, 238u8, 130u8, 90u8, 28u8, 248u8, 68u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::account::Param0,
                    >,
                    types::account::Account,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            0u8, 34u8, 91u8, 238u8, 87u8, 151u8, 107u8, 4u8, 104u8, 111u8, 47u8,
                            9u8, 10u8, 169u8, 173u8, 69u8, 182u8, 183u8, 227u8, 37u8, 148u8, 135u8,
                            184u8, 83u8, 67u8, 66u8, 238u8, 130u8, 90u8, 28u8, 248u8, 68u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::locks::Locks,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        (),
                        [
                            184u8, 159u8, 77u8, 3u8, 45u8, 154u8, 149u8, 191u8, 33u8, 212u8, 190u8,
                            76u8, 222u8, 65u8, 62u8, 163u8, 44u8, 141u8, 151u8, 114u8, 109u8, 98u8,
                            70u8, 103u8, 41u8, 93u8, 14u8, 171u8, 115u8, 127u8, 71u8, 114u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn locks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::locks::Param0,
                    >,
                    types::locks::Locks,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            184u8, 159u8, 77u8, 3u8, 45u8, 154u8, 149u8, 191u8, 33u8, 212u8, 190u8,
                            76u8, 222u8, 65u8, 62u8, 163u8, 44u8, 141u8, 151u8, 114u8, 109u8, 98u8,
                            70u8, 103u8, 41u8, 93u8, 14u8, 171u8, 115u8, 127u8, 71u8, 114u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserves::Reserves,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        (),
                        [
                            242u8, 71u8, 38u8, 101u8, 114u8, 144u8, 32u8, 84u8, 73u8, 233u8, 9u8,
                            119u8, 27u8, 174u8, 152u8, 241u8, 127u8, 145u8, 94u8, 161u8, 79u8,
                            53u8, 143u8, 0u8, 225u8, 6u8, 241u8, 59u8, 43u8, 234u8, 124u8, 161u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn reserves(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::reserves::Param0,
                    >,
                    types::reserves::Reserves,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            242u8, 71u8, 38u8, 101u8, 114u8, 144u8, 32u8, 84u8, 73u8, 233u8, 9u8,
                            119u8, 27u8, 174u8, 152u8, 241u8, 127u8, 145u8, 94u8, 161u8, 79u8,
                            53u8, 143u8, 0u8, 225u8, 6u8, 241u8, 59u8, 43u8, 234u8, 124u8, 161u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::holds::Holds,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        (),
                        [
                            205u8, 23u8, 183u8, 37u8, 107u8, 177u8, 51u8, 246u8, 113u8, 51u8, 33u8,
                            6u8, 87u8, 177u8, 194u8, 231u8, 249u8, 192u8, 51u8, 183u8, 183u8,
                            220u8, 173u8, 61u8, 195u8, 54u8, 221u8, 154u8, 5u8, 19u8, 41u8, 173u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::holds::Param0,
                    >,
                    types::holds::Holds,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            205u8, 23u8, 183u8, 37u8, 107u8, 177u8, 51u8, 246u8, 113u8, 51u8, 33u8,
                            6u8, 87u8, 177u8, 194u8, 231u8, 249u8, 192u8, 51u8, 183u8, 183u8,
                            220u8, 173u8, 61u8, 195u8, 54u8, 221u8, 154u8, 5u8, 19u8, 41u8, 173u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::freezes::Freezes,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        (),
                        [
                            76u8, 71u8, 176u8, 18u8, 64u8, 147u8, 52u8, 135u8, 106u8, 255u8, 248u8,
                            0u8, 109u8, 27u8, 226u8, 178u8, 218u8, 196u8, 37u8, 109u8, 192u8, 49u8,
                            95u8, 1u8, 7u8, 53u8, 84u8, 156u8, 97u8, 64u8, 95u8, 14u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::freezes::Param0,
                    >,
                    types::freezes::Freezes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            76u8, 71u8, 176u8, 18u8, 64u8, 147u8, 52u8, 135u8, 106u8, 255u8, 248u8,
                            0u8, 109u8, 27u8, 226u8, 178u8, 218u8, 196u8, 37u8, 109u8, 192u8, 49u8,
                            95u8, 1u8, 7u8, 53u8, 84u8, 156u8, 97u8, 64u8, 95u8, 14u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u64,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                #[doc = ""]
                #[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_locks(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                #[doc = ""]
                #[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
                pub fn max_reserves(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u32,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet."]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub struct Sudo {
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::minimal_template_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoUncheckedWeight {
                    pub call:
                        ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::minimal_template_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::subxt::ext::subxt_core::alloc::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who = ::subxt::ext::subxt_core::utils::MultiAddress<
                        ::subxt::ext::subxt_core::utils::AccountId32,
                        (),
                    >;
                    pub type Call = runtime_types::minimal_template_runtime::RuntimeCall;
                }
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub struct RemoveKey;
                impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for RemoveKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "remove_key";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                pub fn sudo(
                    &self,
                    call: types::sudo::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::Sudo>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            99u8, 160u8, 122u8, 148u8, 85u8, 201u8, 255u8, 224u8, 112u8, 131u8,
                            96u8, 208u8, 214u8, 198u8, 114u8, 157u8, 144u8, 60u8, 3u8, 28u8, 221u8,
                            226u8, 97u8, 27u8, 105u8, 177u8, 82u8, 222u8, 178u8, 114u8, 139u8,
                            244u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            97u8, 249u8, 24u8, 184u8, 171u8, 202u8, 136u8, 136u8, 221u8, 252u8,
                            43u8, 125u8, 78u8, 54u8, 245u8, 110u8, 211u8, 87u8, 205u8, 75u8, 109u8,
                            107u8, 246u8, 205u8, 79u8, 61u8, 191u8, 175u8, 34u8, 72u8, 25u8, 208u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SetKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::SudoAs>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::subxt::ext::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            46u8, 241u8, 128u8, 156u8, 183u8, 132u8, 169u8, 35u8, 195u8, 252u8,
                            63u8, 253u8, 173u8, 68u8, 238u8, 234u8, 87u8, 2u8, 101u8, 98u8, 223u8,
                            240u8, 86u8, 42u8, 132u8, 230u8, 239u8, 48u8, 251u8, 43u8, 20u8, 103u8,
                        ],
                    )
                }
                #[doc = "Permanently removes the sudo key."]
                #[doc = ""]
                #[doc = "**This cannot be un-done.**"]
                pub fn remove_key(
                    &self,
                ) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<types::RemoveKey>
                {
                    ::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "remove_key",
                        types::RemoveKey {},
                        [
                            133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                            25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
                            191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
                            79u8,
                        ],
                    )
                }
            }
        }
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A sudo call just took place."]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The sudo key has been updated."]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>;
                pub type New = ::subxt::ext::subxt_core::utils::AccountId32;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "The key was permanently removed."]
            pub struct KeyRemoved;
            impl ::subxt::ext::subxt_core::events::StaticEvent for KeyRemoved {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyRemoved";
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt::ext::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key::Key,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        (),
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "The `Event` enum of this pallet"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::ext::subxt_core::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u64;
                pub type Tip = ::core::primitive::u64;
            }
            impl ::subxt::ext::subxt_core::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod next_fee_multiplier {
                    use super::runtime_types;
                    pub type NextFeeMultiplier =
                        runtime_types::sp_arithmetic::fixed_point::FixedU128;
                }
                pub mod storage_version {
                    use super::runtime_types;
                    pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_fee_multiplier::NextFeeMultiplier,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        (),
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::storage_version::StorageVersion,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        (),
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::ext::subxt_core::constants::address::StaticAddress<
                    ::core::primitive::u8,
                > {
                    ::subxt::ext::subxt_core::constants::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod template {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod tuple_storage_map {
                    use super::runtime_types;
                    pub type TupleStorageMap = ::core::option::Option<::core::primitive::u32>;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::core::primitive::u32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn tuple_storage_map_iter(
                    &self,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (),
                    types::tuple_storage_map::TupleStorageMap,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Template",
                        "TupleStorageMap",
                        (),
                        [
                            132u8, 215u8, 87u8, 69u8, 126u8, 40u8, 27u8, 101u8, 74u8, 111u8, 243u8,
                            82u8, 186u8, 40u8, 249u8, 164u8, 10u8, 19u8, 93u8, 70u8, 231u8, 169u8,
                            250u8, 19u8, 36u8, 69u8, 249u8, 210u8, 68u8, 249u8, 124u8, 2u8,
                        ],
                    )
                }
                pub fn tuple_storage_map_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::tuple_storage_map::Param0>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                        types::tuple_storage_map::Param0,
                    >,
                    types::tuple_storage_map::TupleStorageMap,
                    (),
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Template",
                        "TupleStorageMap",
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                            _0.borrow(),
                        ),
                        [
                            132u8, 215u8, 87u8, 69u8, 126u8, 40u8, 27u8, 101u8, 74u8, 111u8, 243u8,
                            82u8, 186u8, 40u8, 249u8, 164u8, 10u8, 19u8, 93u8, 70u8, 231u8, 169u8,
                            250u8, 19u8, 36u8, 69u8, 249u8, 210u8, 68u8, 249u8, 124u8, 2u8,
                        ],
                    )
                }
                pub fn tuple_storage_map(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::tuple_storage_map::Param0>,
                    _1: impl ::core::borrow::Borrow<types::tuple_storage_map::Param1>,
                ) -> ::subxt::ext::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::tuple_storage_map::Param0,
                        >,
                        ::subxt::ext::subxt_core::storage::address::StaticStorageKey<
                            types::tuple_storage_map::Param1,
                        >,
                    ),
                    types::tuple_storage_map::TupleStorageMap,
                    ::subxt::ext::subxt_core::utils::Yes,
                    ::subxt::ext::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt::ext::subxt_core::storage::address::StaticAddress::new_static(
                        "Template",
                        "TupleStorageMap",
                        (
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _0.borrow(),
                            ),
                            ::subxt::ext::subxt_core::storage::address::StaticStorageKey::new(
                                _1.borrow(),
                            ),
                        ),
                        [
                            132u8, 215u8, 87u8, 69u8, 126u8, 40u8, 27u8, 101u8, 74u8, 111u8, 243u8,
                            82u8, 186u8, 40u8, 249u8, 164u8, 10u8, 19u8, 93u8, 70u8, 231u8, 169u8,
                            250u8, 19u8, 36u8, 69u8, 249u8, 210u8, 68u8, 249u8, 124u8, 2u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeakBoundedVec<_0>(pub ::subxt::ext::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Clone,
                            Debug,
                            PartialEq,
                        )]
                        #[allow(non_snake_case)]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[codec(dumb_trait_bound)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                        #[derive(
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                            :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                            :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Clone,
                            Debug,
                            PartialEq,
                        )]
                        #[allow(non_snake_case)]
                        # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                        #[codec(dumb_trait_bound)]
                        #[decode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                        )]
                        #[encode_as_type(
                            crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                        )]
                        pub struct IdAmount<_0, _1> {
                            pub id: _0,
                            pub amount: _1,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "Can be executed by every `origin`."]
                    remark {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    set_code {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
                    #[doc = "version!"]
                    set_code_without_checks {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    do_task {
                        task: runtime_types::minimal_template_runtime::RuntimeTask,
                    },
                    #[codec(index = 9)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
                    #[doc = "later."]
                    #[doc = ""]
                    #[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
                    #[doc = "example that the spec name remains the same and that the version number increases. Not"]
                    #[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
                    #[doc = ""]
                    #[doc = "This call requires Root origin."]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    #[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
                    #[doc = ""]
                    #[doc = "If the authorization required a version check, this call will ensure the spec name"]
                    #[doc = "remains unchanged and that the spec version has increased."]
                    #[doc = ""]
                    #[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
                    #[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
                    #[doc = ""]
                    #[doc = "All origins are allowed."]
                    apply_authorized_upgrade {
                        code: ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                    #[codec(index = 6)]
                    #[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    #[doc = "The specified [`Task`] is not valid."]
                    InvalidTask,
                    #[codec(index = 8)]
                    #[doc = "The specified [`Task`] failed during execution."]
                    FailedTask,
                    #[codec(index = 9)]
                    #[doc = "No upgrade authorized."]
                    NothingAuthorized,
                    #[codec(index = 10)]
                    #[doc = "The submitted code is not authorized."]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::ext::subxt_core::utils::AccountId32,
                        hash: ::subxt::ext::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "A [`Task`] has started executing"]
                    TaskStarted {
                        task: runtime_types::minimal_template_runtime::RuntimeTask,
                    },
                    #[codec(index = 7)]
                    #[doc = "A [`Task`] has finished executing."]
                    TaskCompleted {
                        task: runtime_types::minimal_template_runtime::RuntimeTask,
                    },
                    #[codec(index = 8)]
                    #[doc = "A [`Task`] failed during execution."]
                    TaskFailed {
                        task: runtime_types::minimal_template_runtime::RuntimeTask,
                        err: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 9)]
                    #[doc = "An upgrade was authorized."]
                    UpgradeAuthorized {
                        code_hash: ::subxt::ext::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt::ext::subxt_core::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod minimal_template_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 3)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 3)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 3)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 4)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {}
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeTask {}
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibility of churn)."]
                    upgrade_accounts {
                        who: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    #[doc = "Adjust the total issuance in a saturating way."]
                    #[doc = ""]
                    #[doc = "Can only be called by root and always needs a positive `delta`."]
                    #[doc = ""]
                    #[doc = "# Example"]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    #[doc = "Burn the specified liquid free balance from the origin account."]
                    #[doc = ""]
                    #[doc = "If the origin's account ends up below the existential deposit as a result"]
                    #[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
                    #[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
                    burn {
                        #[codec(compact)]
                        value: ::core::primitive::u64,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    #[doc = "The issuance cannot be modified since it is already deactivated."]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    #[doc = "The delta cannot be zero."]
                    DeltaZero,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::ext::subxt_core::utils::AccountId32,
                        to: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u64 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u64 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u64,
                    },
                    #[codec(index = 21)]
                    #[doc = "The `TotalIssuance` was forcefully changed."]
                    TotalIssuanceForced {
                        old: ::core::primitive::u64,
                        new: ::core::primitive::u64,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    sudo {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::minimal_template_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_unchecked_weight {
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::minimal_template_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    set_key {
                        new: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    sudo_as {
                        who: ::subxt::ext::subxt_core::utils::MultiAddress<
                            ::subxt::ext::subxt_core::utils::AccountId32,
                            (),
                        >,
                        call: ::subxt::ext::subxt_core::alloc::boxed::Box<
                            runtime_types::minimal_template_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    #[doc = "Permanently removes the sudo key."]
                    #[doc = ""]
                    #[doc = "**This cannot be un-done.**"]
                    remove_key,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Error for the Sudo pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account."]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo call just took place."]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The sudo key has been updated."]
                    KeyChanged {
                        old: ::core::option::Option<::subxt::ext::subxt_core::utils::AccountId32>,
                        new: ::subxt::ext::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "The key was permanently removed."]
                    KeyRemoved,
                    #[codec(index = 3)]
                    #[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "[`Config::MinimumPeriod`]."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _None_."]
                    #[doc = ""]
                    #[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
                    #[doc = "that changing the complexity of this call could result exhausting the resources in a"]
                    #[doc = "block to execute any other calls."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::ext::subxt_core::utils::AccountId32,
                        actual_fee: ::core::primitive::u64,
                        tip: ::core::primitive::u64,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u64);
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct OpaqueMetadata(
                pub ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
            );
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::ext::subxt_core::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::subxt::ext::subxt_core::alloc::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Digest {
                        pub logs: ::subxt::ext::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                        :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                        :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Clone,
                        Debug,
                        PartialEq,
                    )]
                    #[allow(non_snake_case)]
                    # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                    #[codec(dumb_trait_bound)]
                    #[decode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                    )]
                    #[encode_as_type(
                        crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                    )]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::ext::subxt_core::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::ext::subxt_core::utils::H256,
                        pub extrinsics_root: ::subxt::ext::subxt_core::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub provides: ::subxt::ext::subxt_core::alloc::vec::Vec<
                        ::subxt::ext::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum ExtrinsicInclusionMode {
                #[codec(index = 0)]
                AllExtrinsics,
                #[codec(index = 1)]
                OnlyInherents,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub impl_name: ::subxt::ext::subxt_core::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt::ext::subxt_core::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                    :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                    :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[allow(non_snake_case)]
                # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
                #[codec(dumb_trait_bound)]
                #[decode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode"
                )]
                #[encode_as_type(
                    crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode"
                )]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
                :: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
                :: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Clone,
                Debug,
                PartialEq,
            )]
            #[allow(non_snake_case)]
            # [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
            #[codec(dumb_trait_bound)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
