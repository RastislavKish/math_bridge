/*
* Copyright (C) 2024 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::{LazyLock, Mutex};

use clap::{Parser, Subcommand};

use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use glib::Propagation;
use gio::ApplicationFlags;

use libmathcat;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
//#[command(propagate_version=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    }

#[derive(Subcommand)]
enum Commands {
    /// Translates the input MathML to string
    Translate,
    /// Shows a browse expression dialog
    Show,
    }

struct KeycodeTranslator {
    map: HashMap<u16, usize>,
    }
impl KeycodeTranslator {

    pub fn new() -> KeycodeTranslator {
        let mut map: HashMap<u16, usize>=HashMap::new();

        let row_1_hardware: &[u16]=
        &[65, 113, 114, 111, 116]; //Space, Left, Right, Up, Down

        let row_1_web: &[usize]=
        &[32, 37, 39, 38, 40];

        let row_2_hardware: &[u16]=
        &[52, 53, 54, 55, 56, 57, 58, 59, 60, 61]; //Z, X, C, V, B, N, M, Comma, dot, Slash

        let row_2_web: &[usize]=
        &[90, 88, 67, 86, 66, 78, 77, 188, 190, 191];

        let row_3_hardware: &[u16]=
        &[38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 51, 36]; //A, S, D, F, G, H, J, K, L, Semicolon, Apostrophe, Backslash, Enter

        let row_3_web: &[usize]=
        &[65, 83, 68, 70, 71, 72, 74, 75, 76, 59, 222, 220, 14];

        let row_4_hardware: &[u16]=
        &[23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35]; //Tab, Q, W, E, R, T, Y, U, I, O, P, Left paren, Right paren

        let row_4_web: &[usize]=
        &[9, 81, 87, 69, 82, 83, 89, 85, 73, 79, 80, 168, 169];

        let row_5_hardware: &[u16]=
        &[49, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]; //Backtick, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, Minus, Equals, Backspace

        let row_5_web: &[usize]=
        &[192, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 173, 61, 8];

        let row_6_hardware: &[u16]=
        &[9, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 95, 96, 118, 119, 110, 115, 112, 117]; //Escape, F1, F2, F3, F4, F5, F6, F, F8, F9, F10, F11, F12, Insert, elete, Home, End, Page up, Page down

        let row_6_web: &[usize]=
        &[27, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 45, 46, 36, 35, 33, 34];

        Self::fill_map(&mut map, &row_1_hardware, &row_1_web);
        Self::fill_map(&mut map, &row_2_hardware, &row_2_web);
        Self::fill_map(&mut map, &row_3_hardware, &row_3_web);
        Self::fill_map(&mut map, &row_4_hardware, &row_4_web);
        Self::fill_map(&mut map, &row_5_hardware, &row_5_web);
        Self::fill_map(&mut map, &row_6_hardware, &row_6_web);

        KeycodeTranslator {
            map,
            }
        }

    pub fn translate(&self, hardware_keycode: u16) -> Option<usize> {
        if self.map.contains_key(&hardware_keycode) {
            return Some(self.map[&hardware_keycode]);
            }

        None
        }

    fn fill_map(map: &mut HashMap<u16, usize>, hardware_row: &[u16], web_row: &[usize]) {
        assert_eq!(hardware_row.len(), web_row.len());

        for (hardware_key, web_key) in hardware_row.iter().zip(web_row.iter()) {
            map.insert(*hardware_key, *web_key);
            }
        }
    }

static SPEECHD: LazyLock<Mutex<speech_dispatcher::Connection>>=LazyLock::new(|| Mutex::new(speech_dispatcher::Connection::open("com.rastislavkish.mathcat_client", "", "", speech_dispatcher::Mode::Threaded).unwrap()));
static KEYCODE_TRANSLATOR: LazyLock<KeycodeTranslator>=LazyLock::new(|| KeycodeTranslator::new());

fn main() -> Result<(), anyhow::Error> {
    libmathcat::set_rules_dir("Rules".to_string()).unwrap();

    let mathml=read_stdin()?;

    if mathml.is_empty() {
        return Ok(());
        }

    libmathcat::set_mathml(mathml).unwrap();

    let cli=Cli::parse();

    match &cli.command {
        Commands::Translate => {
            translate();
            },
        Commands::Show => {
            show();
            }
        }

    Ok(())
    }

fn translate() {
    println!("{}", libmathcat::get_spoken_text().unwrap());
    }
fn show() {
    let application=Application::new(None, ApplicationFlags::HANDLES_OPEN);

    application.connect_open(move |app, _, _| {
        let window=ApplicationWindow::new(app);
        window.set_title("MathCAT");
        window.set_default_size(350, 70);

        window.connect_key_press_event(move |_, key| {

            let keycode=key.hardware_keycode();

            if let Some(web_keycode)=KEYCODE_TRANSLATOR.translate(keycode) {
                let modifiers=key.state()
                .intersection(gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::MOD1_MASK);

                let control=modifiers.contains(gdk::ModifierType::CONTROL_MASK);
                let shift=modifiers.contains(gdk::ModifierType::SHIFT_MASK);
                let alt=modifiers.contains(gdk::ModifierType::MOD1_MASK);

                if let Ok(response)=libmathcat::do_navigate_keypress(web_keycode, shift, control, alt, false) {
                    speak(&response);
                    }
                }

            Propagation::Proceed
            });

        window.show_all();
        });

    application.run();
    }

fn read_stdin() -> Result<String, anyhow::Error> {
    let stdin=io::stdin();

    let mut data: Vec<u8>=Vec::new();
    stdin.lock().read_to_end(&mut data)?;

    Ok(String::from_utf8(data)?)
    }
fn speak(text: &str) {
    let speechd=SPEECHD.lock().unwrap();
    speechd.say(speech_dispatcher::Priority::Text, text);
    }
