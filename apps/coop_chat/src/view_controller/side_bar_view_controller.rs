// // SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// // SPDX-License-Identifier: GPL-3.0-only

// use slint::{ComponentHandle, ModelExt, VecModel};
// use std::rc::Rc;

// use crate::{
//     controller::{ChannelType, SideBarController},
//      App, GroupListViewItem, ListViewItem, SideBarViewAdapter,
// };

// pub struct SideBarViewController {
//     _app: slint::Weak<App>,
//     _controller: Box<dyn SideBarController>,
// }

// impl SideBarViewController {
//     pub fn new(app: &App, controller: Box<dyn SideBarController>) -> Self {
//         let group_channels = controller.group_channels();
//         let group_channels_title = group_channels.text;
//         let direct_channels = controller.direct_channels();
//         let direct_channels_title = direct_channels.text;

//         let group_channels = VecModel::from_slice(&group_channels.channels);
//         let direct_channels = VecModel::from_slice(&direct_channels.channels);

//         let private_icon = app.global::<mi>().get_lock();
//         let public_icon = app.global::<mi>().get_public();
//         let direct_icon = app.global::<mi>().get_account_box();

//         let group_channels = Rc::new(group_channels.map({
//             let private_icon = private_icon.clone();
//             let public_icon = public_icon.clone();
//             let direct_icon = direct_icon.clone();
//             move |c| ListViewItem {
//                 text: c.text.into(),
//                 leading_icon: match c.channel_type {
//                     ChannelType::Private => private_icon.clone(),
//                     ChannelType::Public => public_icon.clone(),
//                     ChannelType::Direct => direct_icon.clone(),
//                 },
//                 ..Default::default()
//             }
//         }));

//         let direct_channels = Rc::new(direct_channels.map(move |c| ListViewItem {
//             text: c.text.into(),
//             leading_icon: match c.channel_type {
//                 ChannelType::Private => private_icon.clone(),
//                 ChannelType::Public => public_icon.clone(),
//                 ChannelType::Direct => direct_icon.clone(),
//             },
//             ..Default::default()
//         }));

//         let channels = Rc::new(VecModel::default());
//         channels.push(GroupListViewItem {
//             text: group_channels_title.into(),
//             items: group_channels.into(),
//         });
//         channels.push(GroupListViewItem {
//             text: direct_channels_title.into(),
//             items: direct_channels.into(),
//         });

//         app.global::<SideBarViewAdapter>()
//             .set_channels(channels.into());

//         Self {
//             _app: app.as_weak(),
//             _controller: controller,
//         }
//     }
// }
