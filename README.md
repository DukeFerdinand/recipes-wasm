# Rust-based Recipe site

A Web Assembly recipe site built using `yew`, powered by [sanity](https://sanity.io).

While I'm not really building this for public usage, I'm making it available for reference and for copying in case you want to build something like this yourself :)

While `yew` hasn't reached a `v1.0` yet (as of time of writing), it's still mature enough for my use case: fetching data over http from a sanity database and displaying it alongside (s)css to make things pretty.

All that being said, please take a look through and feel free to fork and use for whatever use case you'd like :)

### Getting started

You'll need the following packages installed (via `cargo install <package>`):

- `cargo-web`
- `watchexec`

And also dart sass for building the styles. Installation instructions for your platform can be found [here](https://sass-lang.com/install).

No need for configuration on those on OSX and Linux, but I can't speak for Windows.

And for a parallel task runner I'm using the npm package `concurrently` as it's pretty easy to use and I'm a frontend dev so I've got a JavaScript toolchain installed pretty much everywhere.

To run the app the way I do, run `npm install && npm start` or substitute whatever concurrent command you use (the two commands I use are `cargo web start` and `sass styles/index.scss:static/index.css`)

### Features

- [x] Basic App render
- [x] Style building
- [ ] Recipe render _from_ sanity
- [ ] Upload recipe _to_ sanity

---

More to be added here as I add features :)
