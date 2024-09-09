pub mod ally;

use core::panic;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeA {
    Constitution,
    Strength,
    Flexibility,
    Dexterity,
    Agility,
    Insight,
}

impl AttributeA {
    fn iterator() -> Iter<'static, AttributeA> {
        [Self::Constitution, Self::Strength, Self::Flexibility, Self::Dexterity, Self::Agility, Self::Insight].iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RandA {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl RandA {
    fn from_i32(i: i32) -> Self {
        match i {
            3 => Self::S,
            2 => Self::A,
            1 => Self::B,
            0 => Self::C,
            -1 => Self::D,
            -2 => Self::E,
            -3 => Self::F,
            _ => panic!("must set RankA from -3 ~ 3")
        }
    }
}

pub trait CharacterA {
    fn get_attr(attr : AttributeA) -> RandA;
}

pub struct CharacterA01 {
    name : String,
    attr_data : HashMap<AttributeA, RandA>
}

impl CharacterA01 {
    fn new_with_name(name : &str) -> CharacterA01 {
        let mut attr_data = HashMap::new();
        for attr in AttributeA::iterator() {
            attr_data.insert(*attr, RandA::C);
        }
        Self {
            name : name.to_string(),
            attr_data,
        }
    }

    fn set_attr(mut self, attr : AttributeA, value : i32) -> Self {
        let rank = RandA::from_i32(value);
        *self.attr_data.get_mut(&attr).unwrap() = rank;
        self
    }
}

