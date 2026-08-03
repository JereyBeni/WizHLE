//! TouchWiz / Nature UX high-level layer.
//!
//! Provides a visual approximation of a classic TouchWiz home screen
//! used for selecting applications and options within the WizHLE skeleton.

use eframe::egui;

#[derive(Clone, PartialEq)]
pub enum Screen {
    Home,
    AppDrawer,
    Settings,
    About,
}

#[derive(Clone)]
pub struct AppEntry {
    pub name: String,
    pub icon_label: String, // Simple text/emoji icon for the prototype
}

pub struct TouchWizLayer {
    pub current_screen: Screen,
    pub apps: Vec<AppEntry>,
    pub selected_app: Option<String>,
    pub status_message: String,
}

impl TouchWizLayer {
    pub fn new() -> Self {
        let apps = vec![
            AppEntry { name: "Phone".into(), icon_label: "📞".into() },
            AppEntry { name: "Messages".into(), icon_label: "💬".into() },
            AppEntry { name: "Camera".into(), icon_label: "📷".into() },
            AppEntry { name: "Gallery".into(), icon_label: "🖼️".into() },
            AppEntry { name: "Internet".into(), icon_label: "🌐".into() },
            AppEntry { name: "Settings".into(), icon_label: "⚙️".into() },
            AppEntry { name: "Play Store".into(), icon_label: "▶️".into() },
            AppEntry { name: "Clock".into(), icon_label: "🕒".into() },
            AppEntry { name: "Calendar".into(), icon_label: "📅".into() },
            AppEntry { name: "Music".into(), icon_label: "🎵".into() },
            AppEntry { name: "APK Installer".into(), icon_label: "📦".into() },
            AppEntry { name: "Files".into(), icon_label: "📁".into() },
        ];

        Self {
            current_screen: Screen::Home,
            apps,
            selected_app: None,
            status_message: "TouchWiz Home".into(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        // Dark blue-ish TouchWiz-inspired theme
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = egui::Color32::from_rgb(20, 30, 50);
        style.visuals.window_fill = egui::Color32::from_rgb(25, 40, 65);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Status bar
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("WizHLE · TouchWiz").strong().size(14.0));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("100%");
                    ui.label("📶");
                    ui.label("🔋");
                });
            });

            ui.separator();

            match self.current_screen {
                Screen::Home => self.draw_home(ui),
                Screen::AppDrawer => self.draw_app_drawer(ui),
                Screen::Settings => self.draw_settings(ui),
                Screen::About => self.draw_about(ui),
            }

            // Bottom dock / navigation
            ui.add_space(10.0);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("🏠 Home").clicked() {
                    self.current_screen = Screen::Home;
                    self.status_message = "TouchWiz Home".into();
                }
                if ui.button("📱 Apps").clicked() {
                    self.current_screen = Screen::AppDrawer;
                    self.status_message = "Application Drawer".into();
                }
                if ui.button("⚙️ Settings").clicked() {
                    self.current_screen = Screen::Settings;
                    self.status_message = "Settings".into();
                }
            });

            ui.add_space(4.0);
            ui.label(egui::RichText::new(&self.status_message).italics().size(12.0));
        });
    }

    fn draw_home(&mut self, ui: &mut egui::Ui) {
        ui.heading("TouchWiz Home");
        ui.label("Select an application or open the App Drawer.");
        ui.add_space(12.0);

        // Simple 4-column grid of apps (home screen subset)
        let home_apps: Vec<_> = self.apps.iter().take(8).cloned().collect();
        egui::Grid::new("home_grid")
            .num_columns(4)
            .spacing([16.0, 16.0])
            .show(ui, |ui| {
                for (i, app) in home_apps.iter().enumerate() {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(format!("{}
{}", app.icon_label, app.name))
                                    .size(16.0),
                            )
                            .min_size(egui::vec2(90.0, 80.0)),
                        )
                        .clicked()
                    {
                        self.selected_app = Some(app.name.clone());
                        self.status_message = format!("Selected: {}", app.name);

                        if app.name == "Settings" {
                            self.current_screen = Screen::Settings;
                        }
                    }
                    if (i + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });

        if let Some(ref name) = self.selected_app {
            ui.add_space(16.0);
            ui.label(format!("Last selected application: {}", name));
        }
    }

    fn draw_app_drawer(&mut self, ui: &mut egui::Ui) {
        ui.heading("Application Drawer");
        ui.label("All available applications (skeleton list).");
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("drawer_grid")
                .num_columns(4)
                .spacing([12.0, 12.0])
                .show(ui, |ui| {
                    for (i, app) in self.apps.iter().enumerate() {
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new(format!("{}
{}", app.icon_label, app.name))
                                        .size(15.0),
                                )
                                .min_size(egui::vec2(85.0, 75.0)),
                            )
                            .clicked()
                        {
                            self.selected_app = Some(app.name.clone());
                            self.status_message = format!("Launched (placeholder): {}", app.name);

                            if app.name == "Settings" {
                                self.current_screen = Screen::Settings;
                            }
                        }
                        if (i + 1) % 4 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn draw_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.add_space(8.0);

        ui.label("WizHLE Options (skeleton)");
        ui.separator();

        if ui.button("About WizHLE").clicked() {
            self.current_screen = Screen::About;
            self.status_message = "About".into();
        }

        ui.add_space(6.0);
        ui.label("• Architecture: ARM / x86 (planned)");
        ui.label("• 64HLE: 64-bit application support (planned)");
        ui.label("• APK Installer: included in roadmap");
        ui.label("• Current mode: UI prototype only");

        ui.add_space(12.0);
        if ui.button("← Back to Home").clicked() {
            self.current_screen = Screen::Home;
            self.status_message = "TouchWiz Home".into();
        }
    }

    fn draw_about(&mut self, ui: &mut egui::Ui) {
        ui.heading("About WizHLE");
        ui.add_space(8.0);

        ui.label("WizHLE – Experimental TouchWiz High-Level Emulator");
        ui.label("Written in Rust");
        ui.label("This is an educational skeleton project.");
        ui.label("No real Android or TouchWiz emulation is performed yet.");

        ui.add_space(12.0);
        if ui.button("← Back to Settings").clicked() {
            self.current_screen = Screen::Settings;
            self.status_message = "Settings".into();
        }
    }
}
