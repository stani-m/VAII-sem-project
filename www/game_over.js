window.onload = () => {
    let score = document.getElementById("score")
    score.innerText = sessionStorage.getItem("score")
}

function submitRun() {
    if (sessionStorage.getItem("logged in") !== "true") {
        alert("You need to be logged in to submit runs!")
        location.replace("/")
    } else {
        fetch("submit-run", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                user: {
                    username: sessionStorage.getItem("username"),
                    password: sessionStorage.getItem("password")
                },
                run: {
                    score: parseInt(sessionStorage.getItem("score"))
                }
            })
        })
            .then(response => response.text())
            .then(text => {
                let response = JSON.parse(text)
                if (response.code === 0) {
                    sessionStorage.removeItem("score")
                    location.replace("hall-of-fame.html")
                } else {
                    alert(response.body)
                }
            })
    }
}
