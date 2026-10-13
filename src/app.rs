// SPDX-License-Identifier: MPL-2.0

use std::fmt::Write;
use std::time::Duration;

use cosmic::iced::{Alignment, Subscription};
use cosmic::prelude::*;
use cosmic::widget;

use crate::memory::Memory;

pub struct AppModel {
    core: cosmic::Core,
    meminfo: String,
    memory: Option<Memory>,
    label: String,
    tooltip: String,
    icon: widget::icon::Handle,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    SurfaceAction(cosmic::surface::Action<Message>),
}

impl AppModel {
    fn refresh(&mut self) {
        let memory = match crate::memory::read_memory(&mut self.meminfo) {
            Ok(value) => Some(value),
            Err(error) => {
                if self.memory.is_some() || self.label.is_empty() {
                    eprintln!("Cannot read RAM usage: {error}");
                }
                None
            }
        };
        if memory.map(Memory::used_gb) != self.memory.map(Memory::used_gb) || self.label.is_empty()
        {
            self.label.clear();
            match memory.map(Memory::used_gb) {
                Some(value) => {
                    // Writing to a String cannot fail.
                    let _ = write!(self.label, "{value}GB");
                }
                None => self.label.push_str("N/A"),
            }
        }
        if memory.map(|m| (m.used_mb(), m.total_mb()))
            != self.memory.map(|m| (m.used_mb(), m.total_mb()))
            || self.tooltip.is_empty()
        {
            self.tooltip.clear();
            match memory {
                Some(value) => {
                    let _ = write!(self.tooltip, "{}/{} MB", value.used_mb(), value.total_mb());
                }
                None => self.tooltip.push_str("RAM usage unavailable"),
            }
        }
        self.memory = memory;
    }
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.cosmic-ram-applet";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, _flags: ()) -> (Self, Task<cosmic::Action<Message>>) {
        let mut app = Self {
            core,
            meminfo: String::with_capacity(4096),
            memory: None,
            label: String::with_capacity(16),
            tooltip: String::with_capacity(32),
            icon: widget::icon::from_svg_bytes(include_bytes!("../resources/icon.svg"))
                .symbolic(true),
        };
        app.refresh();
        (app, Task::none())
    }

    fn view(&self) -> Element<'_, Message> {
        let applet = &self.core.applet;
        let (major, minor) = applet.suggested_padding(true);
        let padding = if applet.is_horizontal() {
            [minor, major]
        } else {
            [major, minor]
        };
        let mut text = applet.text(&self.label);
        if self.memory.is_some_and(Memory::low_memory) {
            text = text.class(cosmic::theme::Text::Color(
                cosmic::theme::active().cosmic().destructive_color().into(),
            ));
        }
        let content = widget::row([
            widget::icon(self.icon.clone())
                .size(applet.suggested_size(true).0)
                .into(),
            text.into(),
        ])
        .spacing(6)
        .align_y(Alignment::Center);

        applet
            .autosize_window(applet.applet_tooltip(
                widget::container(content).padding(padding),
                self.tooltip.clone(),
                false,
                Message::SurfaceAction,
                self.core.main_window_id(),
            ))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(2)).map(|_| Message::Refresh)
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::Refresh => {
                self.refresh();
                Task::none()
            }
            Message::SurfaceAction(action) => cosmic::surface::surface_task(action),
        }
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
