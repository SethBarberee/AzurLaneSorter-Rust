use crate::find_line;
use crate::read_ships_from_file;

use iced::widget::button::Button;
use iced::widget::column;
use iced::widget::image;
use iced::widget::radio;
use iced::widget::{pick_list, row, Checkbox};
use iced::widget::text;
use iced::{Sandbox, Settings};
use iced::Alignment;
use iced::Length;

use std::collections::HashMap;
use crate::ship::*;

pub struct GUI {
    map: HashMap<i32, Ship>,
    backline: Vec<Ship>,
    frontline: Vec<Ship>,
    subline: Vec<Ship>,
    backline_img: Vec<image::Handle>,
    frontline_img: Vec<image::Handle>,
    subline_img: Vec<image::Handle>,
    import_all: bool, // whether to import all or use the include.txt
    frontline_sort: SortChoice,
    frontline_class_filter: Option<Class>,
    backline_sort: SortChoice,
    backline_class_filter: Option<Class>,
    subline_sort: SortChoice,
    subline_class_filter: Option<Class>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ImportShips,
    SortShips,
    ClearLines,
    FrontlineSort(SortChoice),
    BacklineSort(SortChoice),
    SublineSort(SortChoice),
    ImportAllToggle(bool),
    FrontlineClassFilter(Class),
    BacklineClassFilter(Class),
    SublineClassFilter(Class),
}

impl GUI {
    pub fn new() -> Self {
        let image_test: image::Handle = image::Handle::from_path("test.png");

        Self {
            map: HashMap::new(),
            backline: Vec::new(),
            subline: Vec::new(),
            frontline: Vec::new(),
            import_all: false,
            frontline_sort: SortChoice::HP,
            subline_sort: SortChoice::HP,
            backline_sort: SortChoice::HP,
            frontline_class_filter: None,
            backline_class_filter: None,
            subline_class_filter: None,
            backline_img: vec![image_test.clone(); 3],
            frontline_img: vec![image_test.clone(); 3],
            subline_img: vec![image_test.clone(); 3],
        }

    }

    pub fn start(&self) -> Result<(), iced::Error> {
        GUI::run(Settings::default())
    }
}

impl Sandbox for GUI {
    type Message = Message;

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: iced::Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }

    fn new() -> Self {
        let image_test: image::Handle = image::Handle::from_path("test.png");

        GUI {
            map: HashMap::new(),
            backline: Vec::new(),
            subline: Vec::new(),
            frontline: Vec::new(),
            import_all: false,
            frontline_sort: SortChoice::HP,
            subline_sort: SortChoice::HP,
            backline_sort: SortChoice::HP,
            frontline_class_filter: None,
            backline_class_filter: None,
            subline_class_filter: None,
            backline_img: vec![image_test.clone(); 3],
            frontline_img: vec![image_test.clone(); 3],
            subline_img: vec![image_test.clone(); 3],
        }
    }

    fn title(&self) -> String {
        String::from("Azur Lane Sorter")
    }

    fn update(&mut self, message: Message) {
        match message {
            // TODO: add more stuff here
            Message::SortShips => {

                if self.map.len() > 0
                {
                    (self.backline, self.frontline, self.subline) = find_line(&self.map);
                    // TODO: use controls to actually sort rather than just putting them in the
                    // lines
                    println!("{:?}", self.backline[0]);
                    println!("{:?}", self.backline[1]);
                    println!("{:?}", self.backline[2]);

                    for i in 0..3 {
                        self.backline_img[i] = self.backline[i].retrieve_img();
                    }

                    println!("{:?}", self.frontline[0]);
                    println!("{:?}", self.frontline[1]);
                    println!("{:?}", self.frontline[2]);

                    for i in 0..3 {
                        self.frontline_img[i] = self.frontline[i].retrieve_img();
                    }

                    println!("{:?}", self.subline[0]);
                    println!("{:?}", self.subline[1]);
                    println!("{:?}", self.subline[2]);

                    for i in 0..3 {
                        self.subline_img[i] = self.subline[i].retrieve_img();
                    }

                }
            }
            Message::ImportShips => {
                self.map = read_ships_from_file("data_export.json").unwrap();
            }
            Message::ClearLines => {
                // Reset the lines but don't clear the map
                self.backline = Vec::new();
                self.frontline = Vec::new();
                self.subline = Vec::new();
            }
            Message::FrontlineSort(choice) => self.frontline_sort = choice,
            Message::BacklineSort(choice) => self.backline_sort = choice,
            Message::SublineSort(choice) => self.subline_sort = choice,
            Message::ImportAllToggle(toggle) => self.import_all = toggle,
            Message::FrontlineClassFilter(class) => self.frontline_class_filter = Some(class),
            Message::BacklineClassFilter(class) => self.backline_class_filter = Some(class),
            Message::SublineClassFilter(class) => self.subline_class_filter = Some(class),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // TODO: make the buttons fill the screen
        let controls = column![
            Button::new("Import Ships")
                .on_press(Message::ImportShips)
                .width(Length::Fill)
                .padding(10),
            Button::new("Sort Ships")
                .on_press(Message::SortShips)
                .width(Length::Fill)
                .padding(10),
            Button::new("Clear Lines")
                .on_press(Message::ClearLines)
                .width(Length::Fill)
                .padding(10),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill);


        column![
            controls,
            Checkbox::new("Import All", self.import_all, Message::ImportAllToggle),
            row![
                text("Backline"),
                image::viewer(self.backline_img[0].clone()),
                text("Test"),
                image::viewer(self.backline_img[1].clone()),
                text("Test"),
                image::viewer(self.backline_img[2].clone()),
                text("Test"),
                pick_list(&Class::BACK[..], self.backline_class_filter.clone(), Message::BacklineClassFilter)
            ],
            row(SortChoice::all()
                .iter()
                .copied()
                .map(|sort_choice| {
                    radio(
                        sort_choice,
                        sort_choice,
                        Some(self.backline_sort),
                        Message::BacklineSort,
                    )
                })
                .map(iced::Element::from)
                .collect()),
            row![
                text("Frontline"),
                image::viewer(self.frontline_img[0].clone()),
                image::viewer(self.frontline_img[1].clone()),
                image::viewer(self.frontline_img[2].clone()),
                pick_list(
                    &Class::FRONT[..],
                    self.frontline_class_filter.clone(),
                    Message::FrontlineClassFilter
                )
            ],
            row(SortChoice::all()
                .iter()
                .copied()
                .map(|sort_choice| {
                    radio(
                        sort_choice,
                        sort_choice,
                        Some(self.frontline_sort),
                        Message::FrontlineSort,
                    )
                })
                .map(iced::Element::from)
                .collect()),
            row![
                text("Subline"),
                image::viewer(self.subline_img[0].clone()),
                image::viewer(self.subline_img[1].clone()),
                image::viewer(self.subline_img[2].clone()),
                pick_list(
                    &Class::SUB[..],
                    self.subline_class_filter.clone(),
                    Message::SublineClassFilter
                )
            ],
            row(SortChoice::all()
                .iter()
                .copied()
                .map(|sort_choice| {
                    radio(
                        sort_choice,
                        sort_choice,
                        Some(self.subline_sort),
                        Message::SublineSort,
                    )
                })
                .map(iced::Element::from)
                .collect())
        ]
        .into()
    }
}

