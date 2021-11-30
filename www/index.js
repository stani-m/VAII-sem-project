window.onload = () => {
    let buttonBox = document.getElementById("button_box")
    if (sessionStorage.getItem("logged in") === "true") {
        let manageAccount = document.createElement("a")
        manageAccount.className = "button"
        manageAccount.href = "manage_account.html"
        manageAccount.innerHTML = "Manage account"
        let logOut = document.createElement("a")
        logOut.className = "button"
        logOut.innerHTML = "Log out"
        logOut.onclick = () => {
            sessionStorage.removeItem("logged in")
            location.reload()
        }
        buttonBox.appendChild(manageAccount)
        buttonBox.appendChild(logOut)
    } else {
        let logIn = document.createElement("a")
        logIn.className = "button"
        logIn.href = "log-in.html"
        logIn.innerHTML = "Log in"
        let signUp = document.createElement("a")
        signUp.className = "button"
        signUp.href = "sign-up.html"
        signUp.innerHTML = "Sign up"
        buttonBox.appendChild(logIn)
        buttonBox.appendChild(signUp)
    }
}
