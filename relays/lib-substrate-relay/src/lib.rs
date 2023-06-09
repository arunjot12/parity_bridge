// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! The library of substrate relay. contains some public codes to provide to substrate relay.

#![warn(missing_docs)]

use std::time::Duration;

pub mod conversion_rate_update;
pub mod error;
pub mod finality;
pub mod helpers;
pub mod messages_lane;
pub mod messages_metrics;
pub mod messages_source;
pub mod messages_target;
pub mod on_demand_headers;

/// Default relay loop stall timeout. If transactions generated by relay are immortal, then
/// this timeout is used.
///
/// There are no any strict requirements on block time in Substrate. But we assume here that all
/// Substrate-based chains will be designed to produce relatively fast (compared to the slowest
/// blockchains) blocks. So 1 hour seems to be a good guess for (even congested) chains to mine
/// transaction, or remove it from the pool.
pub const STALL_TIMEOUT: Duration = Duration::from_secs(60 * 60);

/// Transaction creation parameters.
#[derive(Clone, Debug)]
pub struct TransactionParams<TS> {
	/// Transactions author.
	pub signer: TS,
	/// Transactions mortality.
	pub mortality: Option<u32>,
}
