#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{
    Color32,
    CentralPanel,
    Frame,
    Stroke,
    Align,
    Align2,
    FontId,
    Rect,
    Button,
    RichText,
    Id,
    Sense
};
use std::path::{
    PathBuf,
    Path
};
use dirs::home_dir;
use native_dialog::FileDialog;
use system_uri;
use reqwest::blocking;
use zip::ZipArchive;
use zip_extensions::read;
use std::io::Cursor;
use std::fs::{File,create_dir,OpenOptions,create_dir_all,write};
use std::io::{Read,Write};

const bepinexUrl: &str = "https://builds.bepinex.dev/projects/bepinex_be/577/BepInEx_UnityIL2CPP_x64_ec79ad0_6.0.0-be.577.zip";

struct BepInstaller {
    installPathText: String,
    officialModNames: Vec<String>,
    officialModUrls: Vec<String>,
    officialModsChecked: Vec<bool>
}

fn getSavedInstallPath()->String {
    match readConfigFile().as_ref() {
        "" => {
            if cfg!(target_os = "windows"){
                r"C:\Program Files (x86)\Steam\steamapps\common\Crab Game".to_owned()
            } else {
                let home = home_dir();
                if let Some(home_dir) = home {
                    home_dir.join(r".steam/steam/steamapps/common/Crab Game").to_str().unwrap_or_default().to_owned()
                } else {"".to_owned()}
            }
        },
        path => {
            path.to_owned()
        }
    }
}

fn setSavedInstallPath(path: String){
    let configFolder = dirs::config_dir().unwrap().join("Crustacean");
    create_dir_all(&configFolder);
    write(configFolder.join("installPath.cfg"),path);
}

fn readConfigFile()->String {
    std::fs::read_to_string(dirs::config_dir().unwrap().join("Crustacean").join("installPath.cfg")).unwrap_or("".to_owned())
}

impl Default for BepInstaller {
    fn default()->Self{
        if cfg!(target_os = "windows") {
            return Self {installPathText: getSavedInstallPath(), officialModNames: Vec::new(), officialModUrls: Vec::new(), officialModsChecked: Vec::new()}
        } else if cfg!(target_os = "linux"){
            let home = home_dir();
            if let Some(home_dir) = home {
                return Self {installPathText: getSavedInstallPath(), officialModNames: Vec::new(), officialModUrls: Vec::new(), officialModsChecked: Vec::new()}
            }
        } 
        Self {installPathText: "".to_owned(), officialModNames: Vec::new(), officialModUrls: Vec::new(), officialModsChecked: Vec::new()}
    }
}

impl BepInstaller {
    fn new()->Self{
        let mut out = Self::default();
        let client = reqwest::blocking::Client::new();
        let mut response = client.get("https://raw.githubusercontent.com/o7Moon/Crustacean/main/officialModsList")
            .send().expect("failed to download mod list");
        let mut content: String = String::new();
        response.read_to_string(&mut content).expect("failed to read response content to string");
        for line in content.lines() {
            if line.starts_with("#") {continue}
            let split = line.split_once("|").unwrap();
            out.officialModNames.push(split.0.to_owned());
            out.officialModUrls.push(split.1.to_owned());
        }
        out.officialModsChecked.resize(out.officialModNames.len(), false);
        return out;
    }
}

struct ModInstaller {
    installPathText: String,
    url: String,
    filename: String,
}

impl Default for ModInstaller {
    fn default()->Self{
        if cfg!(target_os = "windows") {
            return Self {installPathText: getSavedInstallPath(), url:"".to_owned(), filename:"".to_owned()}
        } else if cfg!(target_os = "linux"){
            let home = home_dir();
            if let Some(home_dir) = home {
                return Self {installPathText: getSavedInstallPath(), url:"".to_owned(), filename:"".to_owned()}
            }
        } 
        Self {installPathText: "".to_owned(),url:"".to_owned(),filename:"".to_owned()}
    }
}

impl ModInstaller {
    fn new(url: &str, filename: &str) -> Self {
        let mut result = Self::default();
        result.url = url.to_owned();
        result.filename = filename.to_owned();
        result
    }
}

fn main() {
    let argopt = std::env::args().nth(1);
    if let Some(arg) = argopt {
        match arg {
            arg if arg.starts_with("crustacean://installMod/") => {
                let url = arg.split_once("//installMod/").unwrap().1;
                let filename = (&url).rsplit_once("/").unwrap().1;
                installModUI(url, filename)
            },
            _ => {
                installBepinexUI()
            }
        }
    } else {
        installBepinexUI()
    }
}

fn installModUI(url: &str, filename: &str){
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0,100.0)),
        max_window_size: Some(egui::vec2(800.0,100.0)),
        ..Default::default()
    };
    let theme = egui::Visuals {
        hyperlink_color: Color32::from_rgb(60,33,24),
        override_text_color: Some(Color32::WHITE),
        ..Default::default()
    };
    let mut app = ModInstaller::new(url,filename);
    eframe::run_native("name", options, Box::new(|_cc| {
        _cc.egui_ctx.set_visuals(theme); 
        Box::new(app)
    }))
}

fn installBepinexUI(){
    let exec: String = std::env::current_exe().unwrap().to_str().unwrap().to_owned();
    let s_uri_app = system_uri::App::new(
        "net.Crustacean.installer".to_owned(),
        "o7Moon".to_owned(),
        "Installer".to_owned(),
        exec,
        None
    );
    system_uri::install(&s_uri_app, &["crustacean".to_owned()]).expect("install failed");
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0,100.0)),
        max_window_size: Some(egui::vec2(700.0,150.0)),
        ..Default::default()
    };
    let theme = egui::Visuals {
        hyperlink_color: Color32::from_rgb(60,33,24),
        override_text_color: Some(Color32::WHITE),
        ..Default::default()
    };
    let mut app = BepInstaller::new();
    eframe::run_native("name", options, Box::new(|_cc| {
        _cc.egui_ctx.set_visuals(theme); 
        Box::new(app)
    }))
}

impl eframe::App for BepInstaller {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut should_close: bool = false;
        custom_window_frame(ctx, _frame, "Crustacean Installer", |ui| {
            ui.horizontal (|ui|{
                ui.label("Install Path: ");
                ui.add(
                    egui::TextEdit::singleline(&mut self.installPathText).hint_text("type your game folder path here").desired_width(400.0)
                );
                if (ui.button("Browse")).clicked(){
                    let dialogResult = FileDialog::new()
                        .set_location(&self.installPathText)
                        .show_open_single_dir();
                    if let Ok(pathopt) = dialogResult{
                        if let Some(path) = pathopt {
                            self.installPathText = path.to_str().unwrap_or_default().to_owned();
                        }
                    }
                }
            });
            ui.add_space(10.0);
            ui.label("Official Mods: ");
            ui.horizontal_wrapped(|ui| {
                for i in 0..self.officialModsChecked.len() {
                    ui.checkbox(&mut self.officialModsChecked[i],&self.officialModNames[i]);
                }
            });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui|{
                if ui.button("Install").clicked() {
                    installBepinex(&self.installPathText);
                    setSavedInstallPath(self.installPathText.to_owned());
                    for i in 0..self.officialModsChecked.len() {
                        if self.officialModsChecked[i] {
                            let pluginsPath: PathBuf = [self.installPathText.to_owned(), "BepInEx".to_owned(), "plugins".to_owned()].iter().collect();
                            create_dir(pluginsPath);
                            let url = self.officialModUrls[i].to_owned();
                            let filename = url.rsplit_once("/").unwrap().1.to_owned();
                            downloadAndInstallMod(self.installPathText.to_owned(), url, filename)
                        }
                    }
                    should_close = true;
                }
            });
        });
        if should_close {_frame.close()}
    }
}

impl eframe::App for ModInstaller {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut should_close = false;
        custom_window_frame(ctx, _frame, "Crustacean | Installing Mod", |ui| {
            ui.horizontal_wrapped(|ui|{
                ui.label("are you sure you want to install: ");
                ui.hyperlink_to(&self.url,&self.url);
                ui.label("?");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui|{
                ui.add_space(10.0);
                if ui.button("wait no dont").clicked() {
                    should_close = true;
                }
                if ui.button("hell yeah").clicked() {
                    downloadAndInstallMod(self.installPathText.to_owned(),self.url.to_owned(),self.filename.to_owned());
                    should_close = true;
                }
            });
        });
        if should_close {_frame.close()}
    }
}

fn downloadAndInstallMod(gamePath: String, url: String, filename: String){
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url)
        .send().expect("failed to download mod");
    let mut content: Vec<u8> = Vec::new();
    response.read_to_end(&mut content);
    
    let file_extension = filename.rsplit_once(".").unwrap().1;
    match file_extension {
        "dll" => {
            let path: PathBuf = [gamePath,"BepInEx".to_owned(),"plugins".to_owned(),filename].iter().collect();
            let mut file = File::create(path).expect("failed to create mod file");
            file.write_all(&content);
        },
        "zip" => {
            let path: PathBuf = [gamePath,"BepInEx".to_owned(),"plugins".to_owned()].iter().collect();
            let mut archive = ZipArchive::new(Cursor::new(content)).expect("failed to create zip file");
            archive.extract(path);
        },
        _ => {
            return
        }
    }
}

fn installBepinex(path: &String) {
    if Path::new(path).join("BepInEx").exists() {
        println!("Crustacean: bepinex exists, skipping download");
        return;
    }
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(bepinexUrl)
        .send().expect("failed to fetch bepinex zip");
    let mut content: Vec<u8> = Vec::new();
    response.read_to_end(&mut content);
    let mut archive = zip::ZipArchive::new(Cursor::new(content)).expect("couldnt read zip");
    archive.extract(path);
}


// adapted from https://github.com/emilk/egui/tree/master/examples/custom_window_frame
fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let text_color = Color32::WHITE;

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                20.0,
                Color32::from_rgb(153, 75, 42),
                Stroke::new(2.0, Color32::from_rgb(208,109,40)),
            );
            let mut title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            painter.rect(
                title_bar_rect,
                5.0,
                Color32::from_rgb(208,109,40),
                Stroke::new(1.0,Color32::from_rgb(208,109,40))
            );
            // Paint the title:
            painter.text(
                rect.center_top() + egui::vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color,
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.right_top() - egui::vec2(30.0,0.0), egui::Vec2::splat(height)),
                Button::new(RichText::new("❎").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.close();
            }

            // Interact with the title bar (drag to move window):
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}