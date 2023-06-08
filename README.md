# VAII-sem-project

This website is a school project I made for web development subject at university. It's a website where you play a game and try to reach the highest score you can. You can submit your score to the public Hall of Fame and compare your score with other players, send them messages and block them from sending you messages.

The game portion of the front-end is written in Rust and compiled to a webassembly module. Back-end is also written in Rust using the [tide](https://github.com/http-rs/tide) framework exposing a REST-like API. [Sqlx](https://github.com/launchbadge/sqlx) is used for communication with database. Any rendering is done inside the front-end using JavaScript.

The game portion of the front-end is deployed here: https://stani-m.github.io/VAII-sem-project/
