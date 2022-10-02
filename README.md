# netplay-bracket-finder
Web page showing upcoming, open registration Melee tournaments with online events.

* https://netplay.djan.world/

[![fetch info from start.gg api](https://github.com/netplay-bracket-finder/netplay-bracket-finder/actions/workflows/build.yml/badge.svg)](https://github.com/netplay-bracket-finder/netplay-bracket-finder/actions/workflows/build.yml)

# implementation

## fetching tournament information with GraphQL + Rust

there is a rust package, `netplay-bracket-finder`, under the `rust/` directory.
it uses `ureq` to fetch JSON from the start.gg GraphQL API (see `rust/src/query.graphql`), and then parses with `serde`.
after some processing, the rust program outputs `events.json` (see `docs/events.json`)

## visualizing events.json with DataTables

the web frontend is served using [github pages](https://pages.github.com), so no hosting costs! i have a CNAME set up (`netplay.djan.world -> netplay-bracket-finder.github.io`).

the frontend is styled with the [bulma](https://bulma.io/) CSS framework. the table sorts filtering, and sorting by columns, thanks to [DataTables](https://www.datatables.net/), a table plugin for jQuery.

## updating the website on a regular basis

there's a scheduled github actions workflow, defined in `.github/workflows/build.yml`. it runs the rust program, stores `events.json`, and commits to the repository.

whenever the repository recieves a new `events.json` commit, the github pages workflow will trigger on push.

so, the site is regularly updated!
