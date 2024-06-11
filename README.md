# Rust/TypeScript Demo

This application provides a demonstration of how to provide a Rust backend and a TypeScript frontend from the same HTTP server.

- The backend uses [axum](https://github.com/tokio-rs/axum), maintained by the same folks responsible for the [tokio](https://tokio.rs/) runtime.
- The frontend uses [React](https://react.dev/) as a sort of Lingua Franca, but the same principles would apply to Vue, etc. Of particular note is using [Recoil](https://recoiljs.org/) to manage state updates and as a demonstration of live updating.
- Styles are provided via [Tailwind CSS](https://tailwindcss.com/).

## Installation & Usage

Ensure you have both Rust and Node installed. Recommended versions are provided via `Cargo.toml` and `.node-version`.

Then, `cargo run` will start the server. Both `HOST` and `PORT` environment variables are supported to bind to a different address, defaulting to `INADDR_ANY` and port `8080`.

Basic tests for the backend are available via `cargo test`.

## Tradeoffs

- For simplicity the build is entirely managed via [Cargo Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html). This could (should?) be split out into multiple processes for development, particularly for anyone that wants first-party support for, say, Tailwind's `--watch` flag. Otherwise, [ripgrep](https://github.com/BurntSushi/ripgrep) and [entr](https://eradman.com/entrproject/) provide a simple method to recompile and relaunch the entire application when source changes:

```
rg --files | entr -rc cargo run
```

- Furthermore, much of the frontend build process involves "create Y file from X file". Consider [Make](https://www.gnu.org/software/make/) for v2.
- Posts in the backend are not assigned a unique ID, nor does the `timestamp` have any amount of validation nor monotonicity. This is more than sufficient for a toy app, but anyone taking this further should immediately back it with a database. Postgres is the best database.
- This demonstrates what I consider to be the simplest integration of Rust and TypeScript, but other options are available. In particular, if the application grows into something resembling a Single-Page Application, consider migrating the React/TS code to a Next.JS application that calls back to Rust via CORS-friendly proxy. MPAs are still simpler to deploy, but until [native page transitions](https://github.com/WICG/view-transitions/blob/main/cross-doc-explainer.md) are available ([Phil Nash is touring a great talk on this now](https://www.youtube.com/watch?v=_uQbAURmKk0)), they will never feel as smooth as their SPA counterparts.

## Where should I go from here?

**This was designed for maximal value for exactly 4 hours of effort.** If you take and extend this, first read through Tradeoffs, above, then:

- The first change you should make is to add end-to-end testing, via [Cypress](https://www.cypress.io/) or [Playwright](https://playwright.dev/). Open the page, ensure you can create a new post, that the post updates live, and that posts persist via a reload.
- Consider adding auth of some variety, and associating the poster with the post. What should be displayed when the user isn't logged in?
- React Suspense is provided, but without a fallback component. You could use a secondary [Recoil](https://recoiljs.org/) atom or React's `use` API to trigger Suspense on post, and provide a fallback to render while initial posts are loading.
- Currently deployment is unconsidered. Docker has a _fanstastic_ [boilerplate Dockerfile for Rust](https://docs.docker.com/language/rust/develop/#get-and-run-the-sample-application), but it won't bring with it the necessary `public` folder. If you want to get that Dockerfile working, you'll need to consider if you want to version-control `public` alongside sources, or integrate NodeJS into the build image of that Dockerfile. Both are solid options with mostly _procedural_ tradeoffs.
- If you want to use another Rust backend framework (like [Gotham](https://gotham.rs/)), this would be a great application to try that with. Aim for parity.

## License

All code—frontend and backend—is made available under the [Mozilla Public License, Version 2](https://www.mozilla.org/en-US/MPL/2.0/).
