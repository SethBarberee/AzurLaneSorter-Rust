use strum_macros::EnumString;
use serde::{Deserialize, Serialize};
use core::fmt;
use iced::widget::image;


#[derive(EnumString, Debug, Clone, Eq, PartialEq, Copy)]
pub enum SortChoice {
    Luck,
    Armor,
    Speed,
    HP,
    Firepower,
    AntiAir,
    Torpedo,
    Evasion,
    Cost,
    Reload,
    AntiSubmarine,
    Oxygen,
    Ammunition,
    Accuracy,
}

impl SortChoice {
    pub fn all() -> [SortChoice; 14] {
        [
            SortChoice::HP,
            SortChoice::Luck,
            SortChoice::Armor,
            SortChoice::Speed,
            SortChoice::Firepower,
            SortChoice::AntiAir,
            SortChoice::Torpedo,
            SortChoice::Evasion,
            SortChoice::Cost,
            SortChoice::Reload,
            SortChoice::AntiSubmarine,
            SortChoice::Oxygen,
            SortChoice::Ammunition,
            SortChoice::Accuracy,
        ]
    }
}

impl From<SortChoice> for String {
    fn from(sort_choice: SortChoice) -> String {
        String::from(match sort_choice {
            SortChoice::HP => "HP",
            SortChoice::Luck => "Luck",
            SortChoice::Armor => "Armor",
            SortChoice::Speed => "Speed",
            SortChoice::Firepower => "Firepower",
            SortChoice::AntiAir => "AntiAir",
            SortChoice::Torpedo => "Torpedo",
            SortChoice::Evasion => "Evasion",
            SortChoice::Cost => "Cost",
            SortChoice::Reload => "Reload",
            SortChoice::AntiSubmarine => "AntiSubmarine",
            SortChoice::Oxygen => "Oxygen",
            SortChoice::Ammunition => "Ammunition",
            SortChoice::Accuracy => "Accuracy",
        })
    }
}


#[derive(Debug, PartialEq, EnumString, Deserialize, Serialize, Clone, Eq, PartialOrd, Ord)]
pub enum Armor {
    Heavy,
    Medium,
    Light,
}

#[derive(Debug, PartialEq, EnumString, Deserialize, Serialize, Clone, Eq, PartialOrd, Ord)]
pub enum Class {
    AE,
    AM,
    AR,
    BB,
    BBV,
    BC,
    BM,
    CA,
    CB,
    CL,
    CV,
    CVL,
    DD,
    IX,
    IXs,
    IXv,
    IXm,
    SS,
    SSV,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Class::AE => write!(f, "AE"),
            Class::AM => write!(f, "AM"),
            Class::AR => write!(f, "AR"),
            Class::BB => write!(f, "BB"),
            Class::BBV => write!(f, "BBV"),
            Class::BC => write!(f, "BBC"),
            Class::BM => write!(f, "BM"),
            Class::CA => write!(f, "CA"),
            Class::CB => write!(f, "CB"),
            Class::CL => write!(f, "CL"),
            Class::CV => write!(f, "CV"),
            Class::CVL => write!(f, "CVL"),
            Class::DD => write!(f, "DD"),
            Class::IX => write!(f, "IX"),
            Class::IXs => write!(f, "IX"),
            Class::IXv => write!(f, "IX"),
            Class::IXm => write!(f, "IX"),
            Class::SS => write!(f, "SS"),
            Class::SSV => write!(f, "SSV"),
        }
    }
}

impl Class {
    pub const SUB: [Class; 4] = [Class::SS, Class::AM, Class::SSV, Class::IX];
    pub const FRONT: [Class; 4] = [Class::CL, Class::CA, Class::CB, Class::DD];
    pub const BACK: [Class; 8] = [
        Class::BB,
        Class::BBV,
        Class::BC,
        Class::BM,
        Class::CV,
        Class::CVL,
        Class::AR,
        Class::AE,
    ];
}

#[derive(Debug, PartialEq, EnumString, Deserialize, Serialize, Clone)]
pub enum ValidLevel {
    Level1,
    Level100,
    Level120,
    Level125,
}



#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub rarity: String,
    pub nation: String,
    pub class: Class,
    pub luck: i32,
    pub armor: Armor,
    pub speed: i32,
    pub hp: i32,
    pub firepower: i32,
    pub antiair: i32,
    pub torpedo: i32,
    pub evasion: i32,
    pub aviation: i32,
    pub cost: i32,
    pub reload: i32,
    pub antisubmarine: i32,
    pub oxygen: i32,
    pub ammunition: i32,
    pub accuracy: i32,
    pub image: String,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl Ship {
    pub fn retrieve_img(&self) -> image::Handle {

        // TODO: actually use the image field and download the ship images
        println!("{}", self.image);

        let image_test: image::Handle = image::Handle::from_path("test.png");

        image_test
    }
}

