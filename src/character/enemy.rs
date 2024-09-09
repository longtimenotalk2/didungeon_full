use super::CharacterA01;

use super::AttributeA::Constitution as Con;
use super::AttributeA::Strength as Str;
use super::AttributeA::Flexibility as Fle;
use super::AttributeA::Dexterity as Dex;
use super::AttributeA::Agility as Agi;
use super::AttributeA::Insight as Ins;

impl CharacterA01 {
    pub fn fighter() -> Self {
        Self::new_with_name("战  士")
        .set_attr(Con, 2)
        .set_attr(Str, 2)
        .set_attr(Fle, -1)
        .set_attr(Agi, -1)
        .set_attr(Ins, -1)
    }

    pub fn thief() -> Self {
        Self::new_with_name("盗  贼")
        .set_attr(Con, -1)
        .set_attr(Str, -1)
        .set_attr(Dex, 1)
        .set_attr(Agi, 2)
    }

    pub fn archer() -> Self {
        Self::new_with_name("弓箭手")
        .set_attr(Dex, 1)
        .set_attr(Ins, 1)
    }

    pub fn ninja() -> Self {
        Self::new_with_name("忍  者")
        .set_attr(Con, -1)
        .set_attr(Fle, 2)
        .set_attr(Dex, 1)
        .set_attr(Agi, 2)
        .set_attr(Ins, 1)
    }
}