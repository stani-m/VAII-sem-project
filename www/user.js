window.onload = () => {
    let lastUsername = sessionStorage.getItem("username");
    if (lastUsername !== null) {
        document.getElementById("username").value = lastUsername
    }
}

function signUp() {
    let username = document.getElementById("username").value
    let password = document.getElementById("password").value
    let repeatPassword = document.getElementById("repeatPassword").value

    if (username === "") {
        alert("Username cannot be empty!")
    } else if (username.length > 20) {
        alert("Username can contain at most 20 characters.")
    } else if (password.length < 8) {
        alert("Password needs to contain at least 8 characters.")
    } else if (password.length > 50) {
        alert("Password needs to contain at most 50 characters.")
    } else if (password !== repeatPassword) {
        alert("Passwords don't match!")
    } else {
        fetch("sign-up", {
            method: "POST",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify({username, password})
        })
            .then(response => response.text())
            .then(text => {
                let response = JSON.parse(text)
                if (response.code === 0) {
                    sessionStorage.setItem("logged in", "true")
                    sessionStorage.setItem("username", username)
                    location.replace("/")
                } else {
                    alert(response.body)
                }
            })
    }
}

function logIn() {
    let username = document.getElementById("username").value
    let password = document.getElementById("password").value

    fetch("log-in", {
        method: "POST",
        headers: {
            "Accept": "application/json",
            "Content-Type": "application/json"
        },
        body: JSON.stringify({username, password})
    })
        .then(response => response.text())
        .then(text => {
            let response = JSON.parse(text)
            if (response.code === 0) {
                sessionStorage.setItem("logged in", "true")
                sessionStorage.setItem("username", username)
                location.replace("/")
            } else {
                alert(response.body)
            }
        })
}
