// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

extern crate nostr;

use std::error::Error;
use std::str::FromStr;

use nostr::event::{Event, EventBuilder};
use nostr::{Keys, Kind, KindBase};

const ALICE_SK: &str = "6b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let alice_keys = Keys::from_str(ALICE_SK)?;

    let difficulty = 20; // leading zero bits
    let msg_content = "This is a Nostr message with embedded proof-of-work";

    let builder = EventBuilder::new(Kind::Base(KindBase::TextNote), msg_content, &[]);
    // or
    // let builder = EventBuilder::new_text_note(msg_content, &[]);

    let event: Event = builder.to_pow_event(&alice_keys, difficulty)?;

    event.verify()?;

    println!("{:#?}", event);

    Ok(())
}