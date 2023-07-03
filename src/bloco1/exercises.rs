use std::io::Stdin;

use crate::exerciser::{ExerciseConfig, ExerciseResult};

use super::{
    e01_class_percentage::{class_percentage, ClassConfig},
    e02_bouquet_price::{bouquet_price, BouquetConfig},
    e03_cilinder_volume::{cilinder_volume, CilinderConfig},
    e04_thunder_distance::{thunder_distance, ThunderConfig},
    e05_building_height::{building_height, BuildingConfig},
    e06_tales_building_height::{tales_building_height, TalesBuildingConfig},
};

pub enum Bloco1Exercises {
    ClassPercentage(ClassConfig),
    BouquetPrice(BouquetConfig),
    CilinderVolume(CilinderConfig),
    ThunderDistance(ThunderConfig),
    BuildingHeight(BuildingConfig),
    TalesBuildingHeight(TalesBuildingConfig),
}

impl Bloco1Exercises {
    pub fn build(exercise: u32, stdin: &Stdin) -> Self {
        match exercise {
            1 => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
            2 => Bloco1Exercises::BouquetPrice(BouquetConfig::build(stdin)),
            3 => Bloco1Exercises::CilinderVolume(CilinderConfig::build(stdin)),
            4 => Bloco1Exercises::ThunderDistance(ThunderConfig::build(stdin)),
            5 => Bloco1Exercises::BuildingHeight(BuildingConfig::build(stdin)),
            6 => Bloco1Exercises::TalesBuildingHeight(TalesBuildingConfig::build(stdin)),
            _ => Bloco1Exercises::ClassPercentage(ClassConfig::build(stdin)),
        }
    }

    pub fn call(self) -> Box<dyn ExerciseResult> {
        match self {
            Bloco1Exercises::ClassPercentage(config) => Box::new(class_percentage(config)),
            Bloco1Exercises::BouquetPrice(config) => Box::new(bouquet_price(config)),
            Bloco1Exercises::CilinderVolume(config) => Box::new(cilinder_volume(config)),
            Bloco1Exercises::ThunderDistance(config) => Box::new(thunder_distance(config)),
            Bloco1Exercises::BuildingHeight(config) => Box::new(building_height(config)),
            Bloco1Exercises::TalesBuildingHeight(config) => Box::new(tales_building_height(config)),
        }
    }
}
