use super::CharacterA01;

use super::AttributeA::Constitution as Con;
use super::AttributeA::Strength as Str;
use super::AttributeA::Flexibility as Fle;
use super::AttributeA::Dexterity as Dex;
use super::AttributeA::Agility as Agi;
use super::AttributeA::Insight as Ins;

impl CharacterA01 {
    pub fn linda() -> Self {
        Self::new_with_name("琳  达")
        .set_attr(Con, 1)
        .set_attr(Fle, 3)
        .set_attr(Dex, 1)
        .set_attr(Agi, -1)
        .set_attr(Ins, -1)
    }

    pub fn yelin() -> Self {
        Self::new_with_name("叶  琳")
        .set_attr(Str, 2)
        .set_attr(Fle, -1)
        .set_attr(Dex, 1)
        .set_attr(Ins, 1)
    }

    pub fn alyssa() -> Self {
        Self::new_with_name("艾丽莎")
        .set_attr(Con, -1)
        .set_attr(Str, -1)
        .set_attr(Fle, 2)
        .set_attr(Dex, 1)
        .set_attr(Agi, 2)
    }

    pub fn elis() -> Self {
        Self::new_with_name("伊丽丝")
        .set_attr(Str, -1)
        .set_attr(Dex, 1)
        .set_attr(Agi, 1)
        .set_attr(Ins, 2)
    }
}