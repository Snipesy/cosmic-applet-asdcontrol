mod asdcontrol_bind;
mod check_envs;
mod hiddev;

use cosmic::app::Core;
use cosmic::iced::{Length, Task};
use cosmic::iced_runtime::core::window::Id as SurfaceId;
use cosmic::widget::{self, slider};
use cosmic::Element;

fn main() -> cosmic::iced::Result {
    check_envs::check_asdcontrol_command();
    cosmic::applet::run::<ASDControlApplet>(())
}

#[derive(Clone)]
struct DisplayState {
    device: String,
    brightness: f32,
    debouncing: bool,
}

struct ASDControlApplet {
    core: Core,
    popup: Option<SurfaceId>,
    displays: Vec<DisplayState>,
}

#[derive(Debug, Clone)]
enum Message {
    TogglePopup,
    PopupClosed(SurfaceId),
    SetBrightness(usize, f32),
    ApplyBrightness(usize, i32),
    NoOp,
}

impl cosmic::Application for ASDControlApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.sznowicki.cosmic-applet-asdcontrol";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let devices = check_envs::check_get_devices();

        let displays = devices
            .into_iter()
            .map(|device| {
                let brightness = asdcontrol_bind::get_bg_value(&device) as f32;
                DisplayState {
                    device,
                    brightness,
                    debouncing: false,
                }
            })
            .collect();

        (
            Self {
                core,
                popup: None,
                displays,
            },
            Task::none(),
        )
    }

    fn on_close_requested(&self, id: SurfaceId) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::TogglePopup => {
                if let Some(id) = self.popup.take() {
                    return cosmic::iced::platform_specific::shell::commands::popup::destroy_popup(
                        id,
                    );
                } else {
                    let id = SurfaceId::unique();
                    self.popup = Some(id);

                    let main_window = match self.core.main_window_id() {
                        Some(win_id) => win_id,
                        None => return Task::none(),
                    };

                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(main_window, id, None, None, None);
                    popup_settings.positioner.size_limits = cosmic::iced::Limits::NONE
                        .min_width(300.0)
                        .max_width(400.0)
                        .min_height(100.0)
                        .max_height(600.0);

                    return cosmic::iced::platform_specific::shell::commands::popup::get_popup(
                        popup_settings,
                    );
                }
            }

            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
            }

            Message::SetBrightness(index, value) => {
                if let Some(display) = self.displays.get_mut(index) {
                    display.brightness = value;

                    if !display.debouncing {
                        display.debouncing = true;

                        return Task::perform(
                            async move {
                                tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                                (index, value.round() as i32)
                            },
                            |(idx, val)| cosmic::Action::App(Message::ApplyBrightness(idx, val)),
                        );
                    }
                }
            }

            Message::ApplyBrightness(index, value) => {
                if let Some(display) = self.displays.get_mut(index) {
                    let device = display.device.clone();
                    let current_value = display.brightness.round() as i32;

                    // Clear debouncing flag
                    display.debouncing = false;

                    // If the value has changed since we started debouncing, schedule a new update
                    if current_value != value {
                        display.debouncing = true;
                        return Task::perform(
                            async move {
                                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                                (index, current_value)
                            },
                            |(idx, val)| cosmic::Action::App(Message::ApplyBrightness(idx, val)),
                        );
                    }

                    // Apply the brightness in a background thread
                    return Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                asdcontrol_bind::set_bg_value(&device, value);
                            })
                            .await
                            .ok();
                        },
                        |_| cosmic::Action::App(Message::NoOp),
                    );
                }
            }

            Message::NoOp => {
                // Do nothing
            }
        }

        Task::none()
    }

    fn view(&'_ self) -> Element<'_, Self::Message> {
        self.core
            .applet
            .icon_button("video-display-symbolic")
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn view_window(&'_ self, id: SurfaceId) -> Element<'_, Self::Message> {
        if self.popup != Some(id) {
            return widget::text("").into();
        }

        let mut content = widget::column().padding(16).spacing(16);

        // Header
        content = content.push(
            widget::text("Brightness Control")
                .size(18)
                .width(Length::Fill),
        );

        // Slider for each display
        for (index, display) in self.displays.iter().enumerate() {
            let display_col = widget::column().spacing(8).width(Length::Fill);

            let label = widget::row()
                .spacing(8)
                .push(
                    widget::text(format!("Display {}", index + 1))
                        .size(14)
                        .width(Length::FillPortion(3)),
                )
                .push(
                    widget::text(format!("{}%", display.brightness.round() as i32))
                        .size(14)
                        .width(Length::FillPortion(1)),
                );

            let brightness_slider = slider(0.0..=100.0, display.brightness, move |value| {
                Message::SetBrightness(index, value)
            })
            .width(Length::Fill);

            content = content.push(display_col.push(label).push(brightness_slider));
        }

        self.core.applet.popup_container(content).into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
