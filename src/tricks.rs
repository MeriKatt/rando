use std::default;
use std::io::Write;

use eframe::egui::*;
use strum::{EnumCount, IntoEnumIterator, VariantIterator};
use crate::{io::*};
type Asset<T> = unreal_asset::Asset<std::io::Cursor<T>>;
use crate::logic::{
 Location,
 Check,
 Lock
};
use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Trick {
 Momentum, // Momentum conservation 
 OneWall, // Single Wall wall-kicks
 ReverseKick, // Reverse wall kicks
 SunsetterAbuse, // Abuse the backflip
 PogoAbuse, // Ascendant Light
 Movement, // General movement such as backflips
 ClingAbuse, // Abuse of cling to climb corners
}

#[derive(strum::EnumString, strum::EnumIter, strum::EnumCount, PartialEq, PartialOrd, strum::Display, Copy, Clone, Debug)]
pub enum Difficulty {
 Disabled,
 Normal,
 Advanced,
 Expert,
 Insane
}

impl Difficulty {
 pub fn from_st(st: String) -> Difficulty {
  if st == "Disabled".to_string() {
   return Difficulty::Disabled;
  } else if st == "Normal".to_string() {
   return Difficulty::Normal;
  } else if st == "Advanced".to_string() {
   return Difficulty::Advanced;
  } else if st == "Expert".to_string() {
   return Difficulty::Expert;
  } else if st == "Insane".to_string() {
   return Difficulty::Insane;
  } else {
   return Difficulty::Disabled;
  }
 }
}

pub struct Tricks {
 pub momentum: Difficulty, // Momentum conservation 
 pub one_wall: Difficulty, // Single Wall wall-kicks
 pub reverse_kick: Difficulty, // Reverse wall kicks
 pub sunsetter_abuse: Difficulty, // Abuse the backflip
 pub pogo_abuse: Difficulty, // Ascendant Light
 pub movement: Difficulty, // General movement such as backflips
 pub cling_abuse: Difficulty, // Abuse of cling to climb corners
}
