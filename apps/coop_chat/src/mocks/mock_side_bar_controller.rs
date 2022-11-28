// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::controller::{Channel, ChannelHeader, ChannelType, SideBarController};

#[derive(Default)]
pub struct MockSideBarController {}

impl MockSideBarController {
    pub fn new() -> Self {
        Self {}
    }
}

impl SideBarController for MockSideBarController {
    fn group_channels(&self) -> ChannelHeader {
        ChannelHeader {
            text: "CHANNELS".into(),
            channels: vec![
                Channel {
                    text: "Town Square".into(),
                    channel_type: ChannelType::Public,
                },
                Channel {
                    text: "Internal".into(),
                    channel_type: ChannelType::Private,
                },
            ],
        }
    }

    fn direct_channels(&self) -> ChannelHeader {
        ChannelHeader {
            text: "DIRECT MESSAGES".into(),
            channels: vec![
                Channel {
                    text: "Town Square".into(),
                    channel_type: ChannelType::Direct,
                },
                Channel {
                    text: "Internal".into(),
                    channel_type: ChannelType::Direct,
                },
            ],
        }
    }
}
