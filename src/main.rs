use reqwest;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    str::FromStr,
};

use terminal_menu::{button, label, menu, mut_menu, run};

use getopts::Options;

pub mod gui;
pub mod ship;

use crate::ship::*;

#[derive(Debug, Clone)]
struct MainState {
    map: HashMap<i32, Ship>,
    backline: Vec<Ship>,
    frontline: Vec<Ship>,
    subline: Vec<Ship>,
    // frontline_sort: SortChoice,
    // frontline_class_filter: Option<Class>,
    // backline_sort: SortChoice,
    // backline_class_filter: Option<Class>,
    // subline_sort: SortChoice,
    // subline_class_filter: Option<Class>,
}


impl MainState {
    fn new() -> Self {
        Self { map: HashMap::new(), backline: Vec::new(), frontline: Vec::new(), subline: Vec::new() }
    }

    fn reset_lines(&mut self) {
        self.frontline = Vec::new();
        self.backline = Vec::new();
        self.subline = Vec::new();
    }

    fn set_map(&mut self, map: HashMap<i32, Ship>) {
        self.map = map;
    }

    fn get_map(&self) -> &HashMap<i32, Ship> {
        &self.map
    }
}

// TODO: beter handling for improper JSON
#[allow(dead_code)]
fn read_ships_from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<i32, Ship>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u: Vec<Ship> = serde_json::from_reader(reader)?;

    let mut map: HashMap<i32, Ship> = HashMap::new();

    let mut index = 0;
    for i in u {
        //println!("{:?}", i);
        map.insert(index, i);
        index += 1;
    }

    Ok(map)
}

fn sort_ships(line: &mut Vec<Ship>, choice: SortChoice) {
    match choice {
        SortChoice::HP => line.sort_by(|a, b| b.hp.cmp(&a.hp)),
        SortChoice::Luck => line.sort_by(|a, b| b.luck.cmp(&a.luck)),
        SortChoice::Armor => line.sort_by(|a, b| b.armor.cmp(&a.armor)),
        SortChoice::Speed => line.sort_by(|a, b| b.speed.cmp(&a.speed)),
        SortChoice::Firepower => line.sort_by(|a, b| b.firepower.cmp(&a.firepower)),
        SortChoice::Cost => line.sort_by(|a, b| b.cost.cmp(&a.cost)),
        SortChoice::Reload => line.sort_by(|a, b| b.reload.cmp(&a.reload)),
        SortChoice::Torpedo => line.sort_by(|a, b| b.torpedo.cmp(&a.torpedo)),
        SortChoice::Evasion => line.sort_by(|a, b| b.evasion.cmp(&a.evasion)),
        SortChoice::AntiSubmarine => line.sort_by(|a, b| b.antisubmarine.cmp(&a.antisubmarine)),
        SortChoice::AntiAir => line.sort_by(|a, b| b.antiair.cmp(&a.antiair)),
        SortChoice::Oxygen => line.sort_by(|a, b| b.oxygen.cmp(&a.oxygen)),
        SortChoice::Ammunition => line.sort_by(|a, b| b.ammunition.cmp(&a.ammunition)),
        SortChoice::Accuracy => line.sort_by(|a, b| b.accuracy.cmp(&a.accuracy)),
    }
}

fn find_line(map: &HashMap<i32, Ship>) -> (Vec<Ship>, Vec<Ship>, Vec<Ship>) {
    let mut backline = Vec::new();
    let mut frontline = Vec::new();
    let mut subline = Vec::new();

    for (_, ship) in map {
        match ship.class {
            Class::BB
            | Class::BBV
            | Class::BC
            | Class::BM
            | Class::CV
            | Class::CVL
            | Class::AR
            | Class::AE => backline.push(ship.clone()),
            Class::CA | Class::CB | Class::CL | Class::DD => frontline.push(ship.clone()),
            Class::SS | Class::AM | Class::SSV | Class::IX | Class::IXs  | Class::IXv | Class::IXm  => subline.push(ship.clone()),
        }
    }
    (backline, frontline, subline)
}

fn filter_line_class(line: &Vec<Ship>, filter: Class) -> Vec<&Ship> { line.iter().filter(|x| x.class == filter).collect() }

fn filter_line_armor(line: &Vec<Ship>, filter: Armor) -> Vec<&Ship> { line.iter().filter(|x| x.armor == filter).collect() }

fn scrape_wiki(level: i32) -> Result<HashMap<i32, Ship>, Box<dyn Error>> {
    let azur_wiki_url = "https://azurlane.koumakan.jp/wiki/List_of_Ships_by_Stats";
    let response = reqwest::blocking::get(azur_wiki_url)?.text()?;


    let mut map = HashMap::new();

    let wiki = response; // full response from wiki
    let mut count = 0;

    let document = scraper::Html::parse_document(&wiki);

    // Select which table we want.. right now we get Level 100 because of 2nd child
    // 1 > 100 > 120 > 125

    let string = match level {
        1 => "article:nth-child(1) > table > tbody ",
        100 => "article:nth-child(2) > table > tbody ",
        120 => "article:nth-child(3) > table > tbody ",
        125 => "article:nth-child(4) > table > tbody ",
        _ => "article:nth-child(1) > table > tbody ",
    };
    let article = scraper::Selector::parse(string).unwrap();

    for rows in document.select(&article) {
        let mut row = rows.text().collect::<Vec<_>>();
        // Remove heading
        row.remove(0); // ID
        row.remove(0); // Ship Name
        row.remove(0); // Rarity
        row.remove(0); // Nation
        row.remove(0); // Type

        let len = row.len(); // number of ships per category
                             //println!("{:?}", len);

        let mut index = 0;
        loop {
            // NOTE: all icons aren't in the header but the data is here
            println!("{:?}", &row[index..index + 19]);

            // build image url for ship
            let mut image_url = String::from("https://azurlane.koumakan.jp/wiki/File:");
            image_url.push_str(
                &row[index + 1]
                    .replace(" (Retrofit)", "Kai")
                    .replace(" ", "_"),
            ); // name of ship
            image_url.push_str("Icon.png");

            let ship = Ship {
                id: String::from(row[index + 0]),
                name: String::from(row[index + 1]),
                rarity: String::from(row[index + 2]),
                nation: String::from(row[index + 3]),
                class: Class::from_str(row[index + 4]).unwrap(),
                luck: row[index + 5].parse().unwrap_or(0),
                armor: Armor::from_str(row[index + 6]).unwrap(),
                speed: row[index + 7].parse().unwrap_or(0),
                hp: row[index + 8].parse().unwrap_or(0),
                firepower: row[index + 9].parse().unwrap_or(0),
                antiair: row[index + 10].parse().unwrap_or(0),
                torpedo: row[index + 11].parse().unwrap_or(0),
                evasion: row[index + 12].parse().unwrap_or(0),
                aviation: row[index + 13].parse().unwrap_or(0),
                cost: row[index + 14].parse().unwrap_or(0),
                reload: row[index + 15].parse().unwrap_or(0),
                antisubmarine: row[index + 16].parse().unwrap_or(0),
                oxygen: row[index + 17].parse().unwrap_or(0),
                ammunition: row[index + 18].parse().unwrap_or(0),
                accuracy: row[index + 19].parse().unwrap_or(0),
                image: image_url,
            };
            //println!("{:?}", ship);
            map.insert(count, ship);
            count += 1;
            index += 20; // 20 fields per row in the table
            if index >= len {
                break;
            };
        }
    }
    Ok(map)
}

#[allow(dead_code)]
fn export_json<P: AsRef<Path>>(path: P, all_lines: &mut Vec<Ship>) -> std::io::Result<()> {
    let output_file = File::create(path)?;
    let mut writer = BufWriter::new(output_file);
    serde_json::to_writer(&mut writer, &all_lines)?;
    writer.flush()?;
    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("g", "gui", "use GUI");
    opts.optflag("h", "help", "help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => todo!(),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return iced::Result::Ok(());
    }

    if matches.opt_present("g") {
       let gui_azur = gui::GUI::new();
       gui_azur.start()
    } else {

        let mut state = MainState::new();
        state.reset_lines();

        loop {

            let main_menu = menu(vec![
                label("1) Read Ships into program"),
                label("2) Sort Ships"),
                label("3) Scrape wiki"),
                button("1"),
                button("2"),
                button("3"),

                button("Quit"),
            ]);

            run(&main_menu);

            match mut_menu(&main_menu).selected_item_name() {
                "1" => {
                    let map3 = read_ships_from_file("data_export.json").unwrap();
                    state.set_map(map3); 
                }
                "2" => {
                    let (backline, frontline, subline) = find_line(state.get_map());

                    for mut line in [backline, frontline, subline] {
                        let selection_menu = menu(vec![
                            label("Select Stat to Sort by:"),
                            button("Luck"),
                            button("Armor"),
                            button("Speed"),
                            button("HP"),
                            button("Firepower"),
                            button("AntiAir"),
                            button("Torpedo"),
                            button("Evasion"),
                            button("Cost"),
                            button("Reload"),
                            button("AntiSubmarine"),
                            button("Oxygen"),
                            button("Ammunition"),
                            button("Accuracy"),
                        ]);

                        run(&selection_menu);

                        sort_ships(
                            &mut line,
                            SortChoice::from_str(mut_menu(&selection_menu).selected_item_name())
                                .unwrap(),
                        );

                        println!("{}", line[0]);
                        println!("{}", line[1]);
                        println!("{}", line[2]);
                    }
                }
                "3" => {
                    let wiki_menu = menu(vec![
                        button("Level1"),
                        button("Level100"),
                        button("Level120"),
                        button("Level125"),
                    ]);

                    run(&wiki_menu);

                    let level =
                        match ValidLevel::from_str(mut_menu(&wiki_menu).selected_item_name())
                            .unwrap()
                        {
                            ValidLevel::Level1 => 1,
                            ValidLevel::Level100 => 100,
                            ValidLevel::Level120 => 120,
                            ValidLevel::Level125 => 125,
                        };

                    let map2 = scrape_wiki(level).unwrap();
                    state.set_map(map2);
                    let (mut backline, mut frontline, mut subline) = find_line(state.get_map());
                    let all_lines = &mut backline;
                    all_lines.append(&mut frontline);
                    all_lines.append(&mut subline);
                    let _ = export_json("data_export.json", all_lines);

                }
                "Quit" => {
                    break;
                }
                _ => {
                    eprint!("Invalid choice");
                    break;
                }
            };
        }
        return iced::Result::Ok(());
    }
}
