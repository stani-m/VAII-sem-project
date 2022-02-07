window.onload = () => {
    let buttonBox = document.getElementById("button_box")
    if (sessionStorage.getItem("logged in") === "true") {
        let myRuns = document.createElement("a")
        myRuns.className = "button"
        myRuns.innerText = "My runs"
        myRuns.onclick = async () => {
            sessionStorage.setItem("view runs", sessionStorage.getItem("username"))
            location.assign("/view-runs.html")
        }
        buttonBox.appendChild(myRuns)
        let myMessages = document.createElement("a")
        myMessages.className = "button"
        myMessages.innerText = "My messages"
        myMessages.href = "view-messages.html"
        buttonBox.appendChild(myMessages)
        let manageAccount = document.createElement("a")
        manageAccount.className = "button"
        manageAccount.href = "manage-account.html"
        manageAccount.innerText = "Manage account"
        buttonBox.appendChild(manageAccount)
        let logOut = document.createElement("a")
        logOut.className = "button"
        logOut.innerText = "Log out"
        logOut.onclick = () => {
            sessionStorage.removeItem("logged in")
            location.reload()
        }
        buttonBox.appendChild(logOut)
    } else {
        let logIn = document.createElement("a")
        logIn.className = "button"
        logIn.href = "log-in.html"
        logIn.innerText = "Log in"
        let signUp = document.createElement("a")
        signUp.className = "button"
        signUp.href = "sign-up.html"
        signUp.innerText = "Sign up"
        buttonBox.appendChild(logIn)
        buttonBox.appendChild(signUp)
    }
}
