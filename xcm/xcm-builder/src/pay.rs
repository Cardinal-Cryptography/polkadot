// Copyright Parity Technologies (UK) Ltd.
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

//! `PayOverXcm` struct for paying through XCM and getting the status back

use frame_support::traits::{
	tokens::{Pay, PaymentStatus},
	Get,
};
use sp_std::{marker::PhantomData, vec};
use xcm::prelude::*;
use xcm_executor::traits::{QueryResponseStatus, XcmQueryHandler};

/// Implementation of the `frame_support_traits::tokens::Pay` trait, to allow
/// for generic payments of a given `AssetKind` and `Balance` from an implied origin, to a
/// beneficiary via XCM, relying on the XCM `TransferAsset` instruction.
///
/// `PayOverXcm::pay` is asynchronous, and returns a `QueryId` which can then be used in
/// `check_payment` to check the status of the XCM transaction.
///
pub struct PayOverXcm<DestChain, Router, Querier, BlockNumber, Timeout>(
	PhantomData<(DestChain, Router, Querier, BlockNumber, Timeout)>,
);
impl<
		DestChain: Get<xcm::v3::MultiLocation>,
		Router: SendXcm,
		Querier: XcmQueryHandler,
		BlockNumber,
		Timeout: Get<Querier::BlockNumber>,
	> Pay for PayOverXcm<DestChain, Router, Querier, BlockNumber, Timeout>
{
	type Beneficiary = MultiLocation;
	type AssetKind = xcm::v3::AssetId;
	type Balance = u128;
	type Id = Querier::QueryId;

	fn pay(
		who: &Self::Beneficiary,
		asset_kind: Self::AssetKind,
		amount: Self::Balance,
	) -> Result<Self::Id, ()> {
		let mut message = Xcm(vec![
			UnpaidExecution { weight_limit: Unlimited, check_origin: None },
			TransferAsset {
				beneficiary: *who,
				assets: vec![MultiAsset { id: asset_kind, fun: Fungibility::Fungible(amount) }]
					.into(),
			},
		]);
		let destination = DestChain::get();
		let id =
			Querier::report_outcome(&mut message, destination, Timeout::get()).map_err(|_| ())?;
		let (ticket, _) =
			Router::validate(&mut Some(destination), &mut Some(message)).map_err(|_| ())?;
		Router::deliver(ticket).map_err(|_| ())?;
		Ok(id)
	}

	fn check_payment(id: Self::Id) -> PaymentStatus {
		match Querier::take_response(id) {
			QueryResponseStatus::Finished { response, at: _ } => match response {
				Response::ExecutionResult(Some(_)) => PaymentStatus::Failure,
				Response::ExecutionResult(None) => PaymentStatus::Success,
				_ => PaymentStatus::Unknown,
			},
			QueryResponseStatus::Pending => PaymentStatus::InProgress,
			QueryResponseStatus::NotFound => PaymentStatus::Unknown,
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_successful(_: &Self::Beneficiary, amount: Self::Balance) {}

	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_concluded(_: Self::Id) {}
}
