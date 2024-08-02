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

// ==UserScript==
// @name math_bridge
// @namespace http://tampermonkey.net/
// @version 0.1
// @description Helps make math content on websites accessible
// @author Rastislav Kish
// @match *://*/*
// @grant none
// @run-at context-menu
// ==/UserScript==

var mathBridgeSocket;
var mathObjects=[];
var mathExpressions=[];

function sendForTranslation(objectIndex) {
    let wrapper=document.createElement("div");
    wrapper.appendChild(mathObjects[objectIndex].cloneNode(true));
    let codeToSend=wrapper.innerHTML;
    mathExpressions.push(codeToSend);

    let msgObj=new Object();
    msgObj.action='translate';
    msgObj.content=codeToSend;

    const msg=JSON.stringify(msgObj);

    mathBridgeSocket.send(msg);
    }

(function() {
    'use strict';

    mathObjects=document.getElementsByTagName("math");
    if (mathObjects.length==0) return;
    mathExpressions=[];

    mathBridgeSocket=new WebSocket('ws://localhost:7513');
    mathBridgeSocket.addEventListener("open", function (e) {
        sendForTranslation(0);
        });
    mathBridgeSocket.addEventListener("message", function (e) {
        const expressionId=mathExpressions.length-1;
        let spanElement=document.createElement("span");
        spanElement.setAttribute("id", `mathExpression${expressionId}`);
        spanElement.textContent=e.data;
        spanElement.onclick=function() {
            let msgObj=new Object();
            msgObj.action='show';
            msgObj.content=mathExpressions[expressionId];

            const msg=JSON.stringify(msgObj);

            mathBridgeSocket.send(msg);
            }

        mathObjects[expressionId].parentNode.replaceChild(spanElement, mathObjects[expressionId]);

        if (mathExpressions.length<mathObjects.legnth) {
            sendForTranslation(expressionId+1);
            }
        });
    mathBridgeSocket.addEventListener("close", function (e) {

        });
    })();
