import {default as fillTable} from "./score-table.js"

window.onload = async () => {
    let view_username = sessionStorage.getItem("view runs")
    document.getElementById("view_username").innerText = view_username
    await fillTable(document.getElementById("score_table"), view_username)
    let username = sessionStorage.getItem("username");
    if (sessionStorage.getItem("view runs") !== null && view_username !== username) {
        let inner = document.getElementById("inner")
        let sendMessage = document.createElement("h3");
        sendMessage.innerText = "Send message:"
        inner.appendChild(sendMessage)
        let form = document.createElement("form")
        let textarea = document.createElement("textarea");
        form.appendChild(textarea)
        let button = document.createElement("button")
        button.className = "button"
        button.innerHTML = "Send"
        button.type = "button"
        button.style.marginLeft = "auto"
        button.style.marginRight = "auto"
        button.onclick = () => {
            let messageText = textarea.value;
            if (username === null) {
                alert("You need to be logged in to send messages.")
            } else if (messageText.length > 500) {
                alert("Message text needs to be 500 characters or less.")
            } else {
                fetch("message", {
                    method: "POST",
                    headers: {
                        "Accept": "application/json",
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({
                        from: {
                            username,
                            password: sessionStorage.getItem("password")
                        },
                        to: {username: view_username},
                        text: messageText
                    })
                })
                    .then(response => response.text())
                    .then(text => {
                        let response = JSON.parse(text)
                        if (response.code === 0) {
                            location.assign("view-messages.html")
                        } else {
                            alert(response.body)
                        }
                    })
            }
        }
        form.appendChild(button)
        inner.appendChild(form)
    }
}