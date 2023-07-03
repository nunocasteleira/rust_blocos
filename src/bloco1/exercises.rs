use std::io::Stdin;

use crate::exerciser::{ExerciseConfig, ExerciseResult};

use super::{
    e01_class_percentage::{class_percentage, ClassConfig},
    e02_bouquet_price::{bouquet_price, BouquetConfig},
    e03_cilinder_volume::{cilinder_volume, CilinderConfig},
    e04_thunder_distance::{thunder_distance, ThunderConfig},
};

pub enum Bloco1Exercises {
    ClassPercentage(ClassConfig),
    BouquetPrice(BouquetConfig),
    CilinderVolume(CilinderConfig),
    ThunderDistance(ThunderConfig),
    // BuildingHeight,
    // TalesBuildingHeight,
}

impl Bloco1Exercises {
    pub fn build(exercise: u32, stdin: &Stdin) -> Self {
        match exercise {
            1 => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
            2 => Bloco1Exercises::BouquetPrice(BouquetConfig::build(stdin)),
            3 => Bloco1Exercises::CilinderVolume(CilinderConfig::build(stdin)),
            4 => Bloco1Exercises::ThunderDistance(ThunderConfig::build(stdin)),
            _ => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
        }
    }

    pub fn call(self) -> Box<dyn ExerciseResult> {
        match self {
            Bloco1Exercises::ClassPercentage(config) => Box::new(class_percentage(config)),
            Bloco1Exercises::BouquetPrice(config) => Box::new(bouquet_price(config)),
            Bloco1Exercises::CilinderVolume(config) => Box::new(cilinder_volume(config)),
            Bloco1Exercises::ThunderDistance(config) => Box::new(thunder_distance(config)),
        }
    }
}
