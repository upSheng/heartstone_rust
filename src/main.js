const {invoke} = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;


import {createApp, ref} from '/vue.esm-browser.js'

createApp({
    setup() {
        const message = ref({
            id: 'container',
            name: 'chs',
            style: 'hh',
            card: {}
        })

        function aa() {
            message.value.name = 'aa'
        }

        function login() {
            invoke('login', {user: 'tauri', password: 'tauri'})
                .then((message) => console.log(message))
                .catch((error) => console.error(error));
        }

        function getData() {
            invoke('get_data', {name: 'tauri'})
                .then((message) => console.log(message))
                .catch((error) => console.error(error));
        }

        function getCardName(name) {
            invoke('get_card_name', {name: name})
                .then((msg) => {
                    console.log(msg);
                    message.value.card = msg;

                })
                .catch((error) => console.error(error));
        }

        return {
            message, aa, login, getData,getCardName
        }
    }
}).mount('#app')

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {name: greetInputEl.value});
}

window.addEventListener("DOMContentLoaded", () => {
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");
    document.querySelector("#greet-form").addEventListener("submit", (e) => {
        e.preventDefault();
        greet();
    });
});
