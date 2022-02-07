window.onload = async () => {
    let username = sessionStorage.getItem("username")
    if (username !== null) {
        let inner = document.getElementById("inner")
        fetch("get-messages", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username,
                password: sessionStorage.getItem("password")
            })
        })
            .then(response => response.text())
            .then(text => {
                let response = JSON.parse(text)
                if (response.code === 0) {
                    let messages = response.body
                    messages.sort((a, b) => (a.time > b.time) ? -1 : ((b.time > a.time) ? 1 : 0))
                    messages.forEach(message => {
                        let sender = message.sender
                        let recipient = message.recipient
                        let time = message.time
                        let message_text = message.text
                        if ((sender === username && message.showSender)
                            || (recipient === username && message.showRecipient)) {
                            let message = document.createElement("div")
                            message.className = "message"
                            let info = document.createElement("p")
                            info.innerHTML = "From: " + sender + "<br>" + "To: " + recipient + "<br>" + "At: " + time
                            message.appendChild(info)
                            let text = document.createElement("p")
                            text.innerText = message_text
                            message.appendChild(text)
                            let reply = document.createElement("a")
                            reply.className = "button"
                            reply.innerText = "Reply"
                            reply.onclick = () => {
                                sessionStorage.setItem("view runs", recipient)
                                location.assign("view-runs.html")
                            }
                            message.appendChild(reply)
                            let hide = document.createElement("a")
                            hide.className = "button"
                            hide.innerText = "Hide"
                            message.appendChild(hide)
                            let block = document.createElement("a")
                            block.className = "button"
                            block.innerText = "Block"
                            message.appendChild(block)
                            inner.appendChild(message)
                        }
                    })
                } else {
                    alert(response.body)
                }
            })
    }
}