// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, Copy, Clone)]
pub enum ChannelType {
    Private,
    Public,
    Direct,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub text: String,
    pub channel_type: ChannelType,
}

#[derive(Debug, Clone)]
pub struct ChannelHeader {
    pub text: String,
    pub channels: Vec<Channel>,
}

pub trait SideBarController {
    fn group_channels(&self) -> ChannelHeader;
    fn direct_channels(&self) -> ChannelHeader;
}
