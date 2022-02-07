function changeUsername() {
    let newUsername = document.getElementById("newUsername").value

    if (newUsername === "") {
        alert("New username cannot be empty!")
    } else if (newUsername.length > 20) {
        alert("New username can contain at most 20 characters.")
    } else {
        let body = {
            newUsername,
            user: {
                username: sessionStorage.getItem("username"),
                password: sessionStorage.getItem("password")
            }
        }
        fetch("change-username", {
            method: "PUT",
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json"
            },
            body: JSON.stringify(body)
        })
            .then(response => response.text())
            .then(text => {
                let response = JSON.parse(text)
                if (response.code === 0) {
                    sessionStorage.setItem("username", newUsername)
                } else {
                    alert(response.body)
                }
            })
    }
}

function changePassword() {
    let newPassword = document.getElementById("newPassword").value

    if (newPassword.length < 8) {
        alert("New password needs to contain at least 8 characters.")
    } else if (newPassword.length > 50) {
        alert("New password needs to contain at most 50 characters.")
    } else {
        let repeatNewPassword = document.getElementById("repeatNewPassword").value

        if (newPassword !== repeatNewPassword) {
            alert("Passwords don't match!")
        } else {
            let oldPassword = document.getElementById("oldPassword").value

            let body = {
                newPassword,
                user: {
                    username: sessionStorage.getItem("username"),
                    password: oldPassword
                }
            }
            fetch("change-password", {
                method: "PUT",
                headers: {
                    "Accept": "application/json",
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(body)
            })
                .then(response => response.text())
                .then(text => {
                    let response = JSON.parse(text)
                    if (response.code === 0) {
                        alert("Password changed successfully.")
                    } else {
                        alert(response.body)
                    }
                })
        }
    }
}

function deleteAccount() {
    let password = document.getElementById("deletePassword").value

    fetch("delete-account", {
        method: "DELETE",
        headers: {
            "Accept": "application/json",
            "Content-Type": "application/json"
        },
        body: JSON.stringify({username: sessionStorage.getItem("username"), password})
    })
        .then(response => response.text())
        .then(text => {
            let response = JSON.parse(text)
            if (response.code === 0) {
                sessionStorage.removeItem("logged in")
                location.replace("/")
            } else {
                alert(response.body)
            }
        })
}