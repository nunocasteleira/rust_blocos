use std::io::Stdin;

use crate::exerciser::{ExerciseConfig, ExerciseResult};

use super::{
    e01_class_percentage::{class_percentage, ClassConfig, ClassResult},
    e02_bouquet_price::{bouquet_price, BouquetConfig, BouquetResult},
    e03_cilinder_volume::{cilinder_volume, CilinderConfig, CilinderResult},
};

pub enum Bloco1Exercises {
    ClassPercentage(ClassConfig),
    BouquetPrice(BouquetConfig),
    CilinderVolume(CilinderConfig),
    // ThunderDistance,
    // BuildingHeight,
    // TalesBuildingHeight,
}

pub enum Bloco1Answers {
    ClassPercentage(ClassResult),
    BouquetPrice(BouquetResult),
    CilinderVolume(CilinderResult),
    // ThunderDistance,
    // BuildingHeight,
    // TalesBuildingHeight,
}

impl Bloco1Exercises {
    pub fn build(exercise: u32, stdin: &Stdin) -> Self {
        match exercise {
            1 => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
            2 => Bloco1Exercises::BouquetPrice(BouquetConfig::build(stdin)),
            3 => Bloco1Exercises::CilinderVolume(CilinderConfig::build(stdin)),
            _ => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
        }
    }

    pub fn call(self) -> Box<dyn ExerciseResult> {
        match self {
            Bloco1Exercises::ClassPercentage(config) => Box::new(class_percentage(config)),
            Bloco1Exercises::BouquetPrice(config) => Box::new(bouquet_price(config)),
            Bloco1Exercises::CilinderVolume(config) => Box::new(cilinder_volume(config)),
        }
    }
}
