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

use std::io::Write;
use std::process::{Command, Stdio};

use derive_getters::Getters;
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::Message;

#[derive(Serialize, Deserialize, Getters)]
struct Request {
    action: String,
    content: String,
    }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let host="0.0.0.0:7513";
    let server=TcpListener::bind(host).await?;
    println!("Server listening on {host}");

    while let Ok((stream, _))=server.accept().await {
        tokio::spawn(communication_thread(stream));
        }

    Ok(())
    }

async fn communication_thread(stream: TcpStream) -> Result<(), anyhow::Error> {
    let ws_stream=tokio_tungstenite::accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver)=ws_stream.split();

    println!("A connection established");

    while let Some(Ok(message))=ws_receiver.next().await {
        if let Message::Text(text)=message {
            println!("Received a message");
            if let Ok(request)=serde_json::from_str::<Request>(&text) {
                println!("Parsed a message");
                match &request.action()[..] {
                    "translate" => {
                        ws_sender.send(Message::Text(translate(request.content()).unwrap())).await?;
                        },
                    "show" => show(request.content())?,
                    _ => {},
                    }
                }
            }
        }

    println!("A connection closed");

    Ok(())
    }

fn translate(mathml: &str) -> Result<String, anyhow::Error> {
    let mut subprocess=Command::new("mathcat_client")
    .arg("translate")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;

    if let Some(mut stdin)=subprocess.stdin.take() {
        stdin.write_all(mathml.as_bytes())?;
        }

    let output=subprocess.wait_with_output()?;

    Ok(String::from_utf8(output.stdout)?)
    }
fn show(mathml: &str) -> Result<(), anyhow::Error> {
    let mut subprocess=Command::new("mathcat_client")
    .arg("show")
    .stdin(Stdio::piped())
    .spawn()?;

    if let Some(mut stdin)=subprocess.stdin.take() {
        stdin.write_all(mathml.as_bytes())?;
        }

    Ok(())
    }

