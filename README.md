# Supreme

[![Test and publish Supreme](https://github.com/opendevtools/supreme-rs/workflows/Test%20and%20publish%20Supreme/badge.svg?branch=main)](https://github.com/opendevtools/supreme-rs/actions?query=workflow%3A%22Test+and+publish+Supreme%22)

This is a rewrite of [Supreme](https://github.com/opendevtools/supreme) in Rust.

![Screenshot of Supreme running in terminal](/docs/supreme.png)

## Install

```
brew install opendevtools/supreme/supreme
```

## Usage

All commands with documentation can be found by running `supreme --help`.

### Add

#### Git

Add a `.gitignore` with some common defaults.

```
supreme add git
```

#### GraphQL Codegen

Add [GraphQL Code Generator](https://graphql-code-generator.com/) and
plugins for generation of TypeScript types.

**Installed npm dependencies:** `graphql`, `@graphql-codegen/cli`, `@graphql-codegen/introspection`, `@graphql-codegen/typescript`, `@graphql-codegen/typescript-resolvers`

```
supreme add graphql-codegen
```

#### Husky

Add a [husky](https://github.com/typicode/husky) configuration containing a pre-commit hook that runs [pretty-quick](https://github.com/azz/pretty-quick). This formats all staged files, that are supported, using [Prettier](http://prettier.io/).

**Installed npm dependencies:** `husky`, `pretty-quick`

```
supreme add husky
```

#### Nvm

Add a `.nvmrc` with the current running Node version

```
supreme add nvm
```

#### Prettier

Add a `.prettierrc` with some defaults.

**Installed npm dependencies:** `prettier`

```
supreme add prettier
```

#### Jest

Install dependencies and configuration for [Jest](https://jestjs.io/) testing. It also adds scripts in `package.json` for running tests locally and in CI environments.

- **Installed npm dependencies (JavaScript):** `jest`, `jest-watch-typeahead`, `is-ci-cli`
- **Installed npm dependencies (ReScript):** `@glennsl/bs-jest`, `jest-watch-typeahead`, `is-ci-cli`

```
supreme add jest
```

#### Tailwind CSS

Install dependencies and configuration for [Tailwind CSS](https://tailwindcss.com/).

- **Installed npm dependencies:** `tailwindcss`, `postcss`, `autoprefixer`

```
supreme add tailwind
```

### Config

#### List

Display the current configuration

```
supreme config list
```

#### Set

Update a configuration setting

- `--node` - Set if you want to use `npm`, `yarn`, or `pnpm` for installing Node
  dependencies [Possible values: `npm`, `yarn`, `pnpm`]

```
supreme config set --node yarn
```

### Install

Install any Node package. Automatically selects `npm`, `yarn`, or `pnpm` depending on
which lockfile exists (or falls back to what's set in the config). If no arguments are passed, it installs all packages in `package.json`.

```
// Install all dependencies
supreme install

// Install one package
supreme install jest

// Install multiple
supreme install jest prettier

// Install with bash expansion
supreme install jest prettier @types/{jest,react}
```

**Tip:** Add aliases for easy access. For instance, `ni` (`alias ni=supreme install`) and `nisd` (`alias nisd=supreme install --dev`).

### GitHub Actions

Create workflows for GitHub actions. It automatically detects these languages
and tweaks the config files:

- JavaScript / TypeScript
- ReScript
- Rust

#### Files created

- `pr_check.yml` - Run tests and linting on pull requests targeting main branch
- `release.yml` - Run tests and publishes a new release on push to main branch

#### Flags

- `--no-npm`, `-n` - Turn off `@semantic-release/npm` in `.releaserc` and remove `NPM_TOKEN` secret from `release.yml`
- `--project`, `-p` - Pass a supported project type [Possible values: `javascript`, `rescript`, `rust`]

#### Environment variables

Environment variables are set inside GitHub repository settings -> Secrets.

- `GITHUB_TOKEN` - Added automatically by GitHub for each repo
- `NPM_TOKEN` (optional) - If you wish to publish the package/app to npm. If you don't want to build to npm then use the `--no-npm` flag.

```
supreme github-actions
```

### ReScript

Create a [ReScript](http://rescript-lang.org/) project with
[Tailwind](https://tailwindcss.com/).

```
supreme rescript my-project-name
```

You'll get some instructions after the project has been created

- `cd my-project-name`
- `npm install`
- `npm start` (start the compiler)
- `npm run server` (in another terminal window, start development server)

### GraphQL

Create a [GraphQL](https://graphql.org/) project.

```
supreme graphql my-project-name
```

You'll get some instructions after the project has been created

- `cd my-project-name`
- `npm install`
- `npm run dev` (start the development server)

### Run

List all available `package.json` `scripts`, select
one and `supreme` will run it.

```
supreme run
```

### Uninstall

Uninstall any Node package. Automatically selects `npm`, `yarn`, or `pnpm` depending on
which lockfile exists (or falls back to what's set in the config).

```
// Uninstall one package
supreme uninstall jest

// Uninstall multiple
supreme uninstall jest prettier

// Uninstall with bash expansion
supreme uninstall jest prettier @types/{jest,react}
```

**Tip:** Add an alias for easy access. For instance, `nu` (`alias nu=supreme uninstall`).

### Update dependencies

Update Node packages. Automatically selects `npm`, `yarn`, or `pnpm` depending on
which lockfile exists (or falls back to what's set in the config).

```
supreme update-dependencies
```

### Workspace

Add or remove packages inside Yarn workspaces.

**Note:** Only works with a `packages` folder for now.

```
// Lists and allows the user to select which package to install to.
// Then prompts the user for a dependency to install.
supreme workspace add

// Lists and allows the user to select which package to install to.
// Installs the provided dependency. Works with bash expansion.
supreme workspace add @types/{jest,node}

// Remove dependency from selected package
supreme workspace remove
```

**Tip:** Add aliases for easy access. For instance, `nw` (`alias nw=supreme workspace add`) and `nwsd` (`alias nwsd=supreme workspace add --dev`)
