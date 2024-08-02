# math_bridge

This is a highly experimental and opinionated project trying to build a decent math support for blind people on Linux. Currently, I have no particularly specific vision of its ideal form. Various technologies have been developed on Windows to make math accessible for people without sight over the recent decades. While some of the ideas people have come up with are interesting and I certainly find them of use, I also do have quite a few strong objections to them. I feel while some of the leads are correct and working, the concept as a whole needs to be rethought, at least for my personal use. I do have multiple ideas on how my ideal math accessibility could look like, but only practice is going to show whether and to what degree are they right and where are their limits.

And that's mostly the purpose of this repository. While the implemented concepts may be of practical use at some point, a lot of the code is going to be toy quality at first to facilitate experimentation. Don't worry, it's likely not going to blow up your PC, just don't expect a super-polished experience. My primary motive is to stimulate discussion about university-level math a11y with practical ready-to-use implementations of suggested concepts, which could hopefully one day grow up to production quality.

One additional note. I don't use math braille, in fact, I can't read braille at all. Therefore do not expect any braille related work or support from me and my code, math can be done fully by speech and that's the state I'm aiming at.

## What's implemented so far

Right now, math_bridge consists of several components. At the core is a daemon program, server providing various math-related functions like MathML2text translation or math tree dialog via the network interface. Each action is handled by a backend, [MathCAT](https://github.com/NSoiffer/MathCAT) at this point, but I'm counting with multiple options. Then, there is a Tampermonkey script for webbrowsers, which upon requests looks up all math elements on a website, sends them for translation and replaces them by a span with the resulting text, while clicking on the element causes a math tree dialog to open.

This structure can change at any point, thus make sure to reread this section or go through release notes to familiarize yourself with the current changes.

## Installation

### dependencies

* [Rust programming language](https://rust-lang.org)
* libspeechd-dev
* Clang
* Git
* Tampermonkey

### Building and setup

```
git clone https://github.com/RastislavKish/math_bridge
cd math_bridge
cd mathcat_client
cargo build --release
cd ..
cd math_daemon
cargo build --release
cd ..
sudo ./install.sh
```

### Setting up Tampermonkey on Firefox

the a11y of TM on Firefox is a bit mixed. Here are few tips to help you set things propelry:

1. Install the script through [Mozilla's addons page](https://addons.mozilla.org). If your browser uses privacy enhancing settings such as no history mode, make sure to enable the extension in private mode during the installation.
2. Go to Tools/Addons, find the Tampermonkey section, click on Other options and select Settings.
3. On the open webpage, find and activate the installed scripts tab, turn of styles temporalily in View/Page style and activate the new script option.
4. Turn styles back into normal in the same menu. Copy the content of tampermonkey_script/math_bridge.js into the edit field, this is the installation media until a more advanced distribution method is figured out. Press Ctrl+S.
5. Additionally, activate the settings tab on the same screen (which is located next to the editor tab), and make sure the script is activated.

After these installation steps, you should see a Tampermonkey submenu with math_bridge option in the context menu on the visited websites. If you don't, try to activate Orca's route the mouse to the cursor command, it should cause the option to appear.

## License

Copyright (C) 2024 Rastislav Kish

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.


