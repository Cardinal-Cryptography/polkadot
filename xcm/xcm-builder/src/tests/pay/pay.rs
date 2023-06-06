// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Tests for making sure `PayOverXcm::pay` generates the correct message and sends it to the correct destination

use super::*;
use frame_support::{assert_ok, traits::tokens::Pay};

type AccountId = u64;
type BlockNumber = u64;

/// Type representing both a location and an asset that is held at that location.
/// The id of the held asset is relative to the location where it is being held.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
pub struct AssetKind {
	destination: MultiLocation,
	asset_id: AssetId,
}

pub struct LocatableAssetKindConverter;
impl sp_runtime::traits::Convert<AssetKind, LocatableAssetId> for LocatableAssetKindConverter {
	fn convert(value: AssetKind) -> LocatableAssetId {
		LocatableAssetId { asset_id: value.asset_id, location: value.destination }
	}
}

/// Simple converter to turn a u64 into a MultiLocation using the AccountIndex64 junction and no parents
pub struct AliasesIntoAccountIndex64;
impl<'a> sp_runtime::traits::Convert<&'a AccountId, MultiLocation> for AliasesIntoAccountIndex64 {
	fn convert(who: &AccountId) -> MultiLocation {
		AccountIndex64 { network: None, index: who.clone().into() }.into()
	}
}

parameter_types! {
	pub InteriorAccount: InteriorMultiLocation = AccountIndex64 { index: 3u64, network: None }.into();
	pub InteriorBody: InteriorMultiLocation = Plurality { id: BodyId::Treasury, part: BodyPart::Voice }.into();
	pub Timeout: BlockNumber = 5; // 5 blocks
	pub Beneficiary: AccountId = 5;
}

/// Scenario:
/// Account #3 on the local chain, Parachain(42), controls an amount of funds on Parachain(2).
/// PayOverXcm::pay creates the correct message for account #3 to pay another account, account #5, on Parachain(2), remotely, in its native token.
#[test]
fn pay_over_xcm_works() {
	let who = 5u64;
	let asset_kind =
		AssetKind { destination: (Parent, Parachain(2)).into(), asset_id: Here.into() };
	let amount = 1000u128;

	assert_ok!(PayOverXcm::<
		InteriorAccount,
		TestMessageSender,
		TestQueryHandler<TestConfig, BlockNumber>,
		Timeout,
		AccountId,
		AssetKind,
		LocatableAssetKindConverter,
		AliasesIntoAccountIndex64,
	>::pay(&who, asset_kind, amount));

	let expected_message = Xcm(vec![
		DescendOrigin(AccountIndex64 { index: 3, network: None }.into()),
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		SetAppendix(Xcm(vec![ReportError(QueryResponseInfo {
			destination: (Parent, Parachain(42)).into(),
			query_id: 1,
			max_weight: Weight::zero(),
		})])),
		TransferAsset {
			assets: (Here, 1000u128).into(),
			beneficiary: AccountIndex64 { index: 5, network: None }.into(),
		},
	]);

	let expected_hash = fake_message_hash(&expected_message);
	assert_eq!(sent_xcm(), vec![((Parent, Parachain(2)).into(), expected_message, expected_hash)]);
}

/// Scenario:
/// A pluralistic body, a Treasury, on the local chain, Parachain(42), controls an amount of funds on Parachain(2).
/// PayOverXcm::pay creates the correct message for the treasury to pay another account, account #7, on Parachain(2), remotely, in the relay's token.
#[test]
fn pay_over_xcm_governance_body() {
	let who = 7u64;
	let asset_kind =
		AssetKind { destination: (Parent, Parachain(2)).into(), asset_id: Parent.into() };
	let amount = 500u128;

	assert_ok!(PayOverXcm::<
		InteriorBody,
		TestMessageSender,
		TestQueryHandler<TestConfig, BlockNumber>,
		Timeout,
		AccountId,
		AssetKind,
		LocatableAssetKindConverter,
		AliasesIntoAccountIndex64,
	>::pay(&who, asset_kind, amount));

	let expected_message = Xcm(vec![
		DescendOrigin(Plurality { id: BodyId::Treasury, part: BodyPart::Voice }.into()),
		UnpaidExecution { weight_limit: Unlimited, check_origin: None },
		SetAppendix(Xcm(vec![ReportError(QueryResponseInfo {
			destination: (Parent, Parachain(42)).into(),
			query_id: 1,
			max_weight: Weight::zero(),
		})])),
		TransferAsset {
			assets: (Parent, 500u128).into(),
			beneficiary: AccountIndex64 { index: 7, network: None }.into(),
		},
	]);
	let expected_hash = fake_message_hash(&expected_message);
	assert_eq!(sent_xcm(), vec![((Parent, Parachain(2)).into(), expected_message, expected_hash)]);
}
