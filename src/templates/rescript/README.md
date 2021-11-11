# {{ name }}

## Run Project

```sh
npm install
npm start
# in another tab
npm run server
```

When both processes are running, open a browser at http://localhost:3000

## Build for Production

```sh
npm run clean
npm run build
npm run build:production
```

This will replace the development artifacts, compile the ReScript code as well as copy `public/index.html` into `dist/`. You can then deploy the contents of the `dist` directory (`index.html` and `Index.js`).

If you make use of routing (via `ReasonReact.Router` or similar logic) ensure
that server-side routing handles your routes or that 404's are directed back to
`index.html` (which is how the dev server is set up).
