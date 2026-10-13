// SPDX-License-Identifier: MPL-2.0

mod app;
mod memory;

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<app::AppModel>(())
}
