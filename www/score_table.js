window.addEventListener("load", async () => {
    let score_table = document.getElementById("full_score_table")
    let thead = document.createElement("thead")
    let tr = document.createElement("tr")
    thead.appendChild(tr)
    let place = document.createElement("th")
    place.innerText = "Place"
    tr.appendChild(place)
    let username = document.createElement("th")
    username.innerText = "Username"
    tr.appendChild(username)
    let score = document.createElement("th")
    score.innerText = "Score"
    tr.appendChild(score)
    let time = document.createElement("th")
    time.innerText = "Time"
    tr.appendChild(time)

    score_table.innerText = ""
    score_table.appendChild(thead)

    let runs = await fetchRuns();
    runs = runs.sort((a, b) => b.score - a.score)
    let current_place = 1;
    let current_score = runs[0].score
    let run_elements = runs.map((run) => {
        if (run.score < current_score) {
            current_place++
            current_score = run.score
        }
        let run_element = document.createElement("tr")
        let place = document.createElement("td")
        place.innerText = current_place.toString()
        run_element.appendChild(place)
        let username = document.createElement("td")
        username.innerText = run.username
        run_element.appendChild(username)
        let score = document.createElement("td")
        score.innerText = run.score
        run_element.appendChild(score)
        let time = document.createElement("td")
        time.innerText = run.time
        run_element.appendChild(time)
        return run_element
    })

    let tbody = document.createElement("tbody")
    for (const run of run_elements) {
        tbody.appendChild(run)
    }
    score_table.appendChild(tbody)
})

async function fetchRuns(username = null) {
    let body = {}
    if (username !== null) {
        body = {
            user: {
                username
            }
        }
    }
    return await fetch("get-runs", {
        method: "POST",
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
                return response.body
            } else {
                alert(response.body)
            }
        })
}
