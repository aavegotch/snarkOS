// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod config;
pub use config::Config;

pub mod connections;
pub use connections::{Connection, ConnectionSide};

mod known_peers;
pub use known_peers::KnownPeers;

mod stats;
pub use stats::Stats;

use tracing::{debug_span, error_span, info_span, trace_span, warn_span, Span};

// FIXME: this can probably be done more elegantly
/// Creates the Tcp's tracing span based on its name.
pub fn create_span(tcp_name: &str) -> Span {
    let mut span = trace_span!("tcp", name = tcp_name);
    if !span.is_disabled() {
        return span;
    } else {
        span = debug_span!("tcp", name = tcp_name);
    }
    if !span.is_disabled() {
        return span;
    } else {
        span = info_span!("tcp", name = tcp_name);
    }
    if !span.is_disabled() {
        return span;
    } else {
        span = warn_span!("tcp", name = tcp_name);
    }
    if !span.is_disabled() { span } else { error_span!("tcp", name = tcp_name) }
}
