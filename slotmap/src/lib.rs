#![cfg_attr(all(not(test), not(debug_assertions)), no_std)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Slot {
    index: usize,
    generation: usize,
}

#[derive(Debug, Default)]
pub struct SlotMap<T> {
    slots: Vec<Slot>,
    data: Vec<T>,
    free: Vec<usize>,
}

// BLAME(usman): This code is equivalent to the tested code. Can be verified by swapping
// configuration attributes with the below `impl` block.
// #[cfg(test)]
#[cfg(all(not(test), not(debug_assertions)))]
impl<T> SlotMap<T> {
    pub fn delete(&mut self, slot: Slot) -> Option<T> {
        let data = &mut self.data;
        let free = &mut self.free;

        self.slots
            .get_mut(slot.index)
            .filter(|internal| internal.generation == slot.generation)
            .and_then(move |valid| {
                data.get_mut(valid.index).map(|value| {
                    free.push(slot.index);
                    valid.generation += 1;

                    let zeroed = unsafe { core::mem::zeroed() };
                    core::mem::replace(value, zeroed)
                })
            })
    }

    pub fn get(&self, slot: Slot) -> Option<&T> {
        self.slots
            .get(slot.index)
            .filter(|internal| internal.generation == slot.generation)
            .and_then(|valid| self.data.get(valid.index))
    }

    pub fn get_mut(&mut self, slot: Slot) -> Option<&mut T> {
        let data = &mut self.data;

        self.slots
            .get_mut(slot.index)
            .filter(|internal| internal.generation == slot.generation)
            .and_then(move |valid| data.get_mut(valid.index))
    }

    pub fn insert(&mut self, value: T) -> Slot {
        match self.free.first() {
            Some(&index) => {
                self.free.swap_remove(0);
                let internal = self.slots[index];
                self.data[internal.index] = value;

                Slot {
                    index,
                    generation: internal.generation,
                }
            },
            None => {
                let generation = 0;
                let internal = Slot {
                    index: self.data.len(),
                    generation,
                };
                self.slots.push(internal);
                self.data.push(value);

                Slot {
                    index: self.slots.len() - 1,
                    generation
                }
            },
        }
    }
}

// #[cfg(not(test))]
#[cfg(any(test, debug_assertions))]
impl<T> SlotMap<T> {
    pub fn delete(&mut self, slot: Slot) -> Option<T> {
        let internal = self.slots.get_mut(slot.index).expect(&format!(
            "Failed to find internal slot for external slot with index {}",
            slot.index
        ));

        if internal.generation == slot.generation {
            let data = self.data.get_mut(internal.index).expect(&format!(
                "Failed to find data for internal slot with index {}",
                internal.index
            ));

            self.free.push(slot.index);
            internal.generation += 1;

            let zeroed = unsafe { core::mem::zeroed() };
            Some(core::mem::replace(data, zeroed))
        } else {
            None
        }
    }

    pub fn get(&self, slot: Slot) -> Option<&T> {
        // Indexing will panic when external slot indexing is improper
        let internal = self.slots.get(slot.index).expect(&format!(
            "Failed to find external slot at index {}",
            slot.index
        ));
        if internal.generation == slot.generation {
            // Indexing will panic when internal slot indexing is improper
            let data = self.data.get(internal.index).expect(&format!(
                "Failed to find data for internal slot with index {}",
                internal.index
            ));
            Some(data)
        } else {
            // The generation of the internal slot is different to the generation of the requested
            // slot. This is the only valid way we should return `None`. All other paths are bugs!
            None
        }
    }

    pub fn get_mut(&mut self, slot: Slot) -> Option<&mut T> {
        let internal = self.slots.get(slot.index).expect(&format!(
            "Failed to find external slot at index {}",
            slot.index
        ));
        if internal.generation == slot.generation {
            let s = self.data.get_mut(internal.index).expect(&format!(
                "Failed to find data for internal slot at index {}",
                internal.index
            ));
            Some(s)
        } else {
            // The generation of the internal slot is different to the generation of the requested
            // slot. This is the only valid way we should return `None`. All other paths are bugs!
            None
        }
    }

    pub fn insert(&mut self, value: T) -> Slot {
        match self.free.first() {
            Some(&index) => {
                self.free.swap_remove(0);
                let internal = self
                    .slots
                    .get(index)
                    .expect(&format!("Failed to get slot at index {}", index));
                let data = self.data.get_mut(internal.index).expect(&format!(
                    "Failed to find data for internal slot with index {}",
                    internal.index
                ));
                *data = value;

                Slot {
                    index,
                    generation: internal.generation,
                }
            }
            None => {
                let generation = 0;
                let external = Slot {
                    index: self.slots.len(),
                    generation,
                };
                let internal = Slot {
                    index: self.data.len(),
                    generation,
                };

                self.slots.push(internal);
                self.data.push(value);
                external
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one() -> (Slot, SlotMap<u32>) {
        let mut sm = SlotMap::default();
        let s = sm.insert(1);

        (s, sm)
    }

    #[test]
    fn insert_into_empty() {
        let (one, _) = one();
        assert_eq!(
            one,
            Slot {
                index: 0,
                generation: 0
            }
        );
    }

    #[test]
    fn get() {
        let (s, sm) = one();
        assert_eq!(sm.get(s), Some(&1));
    }

    #[test]
    fn get_mut() {
        let (s, mut sm) = one();
        assert_eq!(sm.get_mut(s), Some(&mut 1));
    }

    #[test]
    fn mutate_and_get() {
        let (s, mut sm) = one();
        sm.get_mut(s).map(|v| *v = 2);

        assert_eq!(sm.get(s), Some(&2));
    }

    #[test]
    fn delete() {
        let (s, mut sm) = one();
        assert_eq!(sm.delete(s), Some(1));
        assert_eq!(sm.free.len(), 1);
        assert_eq!(sm.slots[s.index].generation, 1);
    }

    #[test]
    fn double_delete() {
        let (s, mut sm) = one();
        sm.delete(s);

        assert_eq!(sm.delete(s), None);
    }

    #[test]
    fn delete_get() {
        let (s, mut sm) = one();
        sm.delete(s);

        assert_eq!(sm.get(s), None);
    }

    #[test]
    fn delete_insert() {
        let (s, mut sm) = one();
        sm.delete(s);
        let ns = sm.insert(2);
        assert_eq!(
            ns,
            Slot {
                index: 0,
                generation: 1
            }
        );
        assert_eq!(sm.free.len(), 0);
        assert_eq!(sm.get(ns), Some(&2));
    }

    #[test]
    fn full_usage() {
        let mut sm = SlotMap::default();
        let one = sm.insert(1);

        // First we need to immutably retrieve `one`
        assert_eq!(sm.get(one), Some(&1));

        // Next we need to modify `one`
        {
            let val = sm.get_mut(one);
            assert_eq!(val, Some(&mut 1));
            val.map(|v| *v = 2);
        }

        // Need to retrieve the new value
        assert_eq!(sm.get(one), Some(&2));

        // Add another element!
        let three = sm.insert(3);
        assert_eq!(
            three,
            Slot {
                index: 1,
                generation: 0
            }
        );

        // Delete `one`, we don't need it anymore
        let one_value = sm.delete(one);
        assert_eq!(one_value, Some(2));

        // Insert another element!
        let four = sm.insert(4);
        assert_eq!(
            four,
            Slot {
                index: 0,
                generation: 1
            }
        );
    }
}
