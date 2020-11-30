// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::prelude::*;

use core::str::FromStr;

const UTXO_INPUT: &str = "52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00";

#[test]
fn from_to_str() {
    assert_eq!(UTXO_INPUT, UTXOInput::from_str(UTXO_INPUT).unwrap().to_string());
}
