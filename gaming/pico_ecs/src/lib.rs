// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

// non working example code

pub struct EntityBuilder<'a, const N: usize> {
    world: &'a mut World<N>,
}

impl<'a, const N: usize> EntityBuilder<'a, N> {
    pub fn insert<T>(self, _component: T) -> Self {
        self
    }

    pub fn id(self) -> u8 {
        self.world.current_id
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct World<const N: usize> {
    current_id: u8,
    storage: [u8; N],
}

impl<const N: usize> Default for World<N> {
    fn default() -> Self {
        Self {
            current_id: Default::default(),
            storage: [0; N],
        }
    }
}

impl<const N: usize> World<N> {
    pub fn spawn(&mut self) -> EntityBuilder<N> {
        self.current_id += 1;

        EntityBuilder { world: self }
    }
}

fn _test() {
    struct Player;

    let mut world = World::<2000>::default();

    let _entity = world.spawn().insert(Player).id();
}
