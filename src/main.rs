use cosmic::iced::widget::{Column, Text};
use cosmic::{app, widget, Action, Element, Task};
use std::fs;

const APP_ID: &str = "dev.naktix.Charge";
const TEXT_SIZE: u16 = 12;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<ChargeApplet>(())
}

struct ChargeApplet {
    core: app::Core,
    percent: String,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    UpdatePercent(String),
}

impl cosmic::Application for ChargeApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut app::Core {
        &mut self.core
    }

    fn init(core: app::Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let app = ChargeApplet {
            core,
            percent: "??".into(),
        };
        let task = Task::perform(read_battery(), |p| Action::App(Message::UpdatePercent(p)));
        (app, task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::Tick => {
                Task::perform(read_battery(), |p| Action::App(Message::UpdatePercent(p)))
            }
            Message::UpdatePercent(p) => {
                self.percent = p;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        cosmic::iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = Column::new()
            .push(widget::text(&self.percent).size(TEXT_SIZE))
            .align_x(cosmic::iced::alignment::Horizontal::Center);

        widget::container(content)
            .width(cosmic::iced::Length::Fill)
            .height(cosmic::iced::Length::Fill)
            .center_x(cosmic::iced::Length::Fill)
            .center_y(cosmic::iced::Length::Fill)
            .into()
    }

    //Dummy view_window
    fn view_window(&self, _id: cosmic::iced::window::Id) -> Element<'_, Self::Message> {
        Text::new("").into()
    }
}

async fn read_battery() -> String {
    //Verfügbare Batterien
    let paths = [
        "/sys/class/power_supply/BAT0/capacity",
        "/sys/class/power_supply/BAT1/capacity",
    ];
    let mut capacities = Vec::new();

    for path in &paths {
        if let Ok(val) = fs::read_to_string(path) {
            if let Ok(num) = val.trim().parse::<u8>() {
                capacities.push(num);
            }
        }
    }

    if capacities.is_empty() {
        return "??".into();
    }

    if capacities.len() == 2 {
        return format!("{}%\n{}%", capacities[0], capacities[1]);
    }

    capacities[0].to_string()
}
