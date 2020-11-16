# Supreme

[![Test and publish Supreme](https://github.com/opendevtools/supreme-rs/workflows/Test%20and%20publish%20Supreme/badge.svg?branch=main)](https://github.com/opendevtools/supreme-rs/actions?query=workflow%3A%22Test+and+publish+Supreme%22)

This is a rewrite of [Supreme](https://github.com/opendevtools/supreme) in Rust.

![Screenshot of Supreme running in terminal](/docs/supreme.png)

## Install

```
brew tap opendevtools/supreme
brew install supreme
```

## Usage

All commands with documentation can be found by running `supreme --help`.

### Add

#### Git

Add a `.gitignore` with some common defaults.

```
supreme add git
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

### GitHub Actions

Create workflows for GitHub actions.

- `pr_check.yml` - Run tests and linting on pull requests targeting master branch
- `release.yml` - Run tests and publishes a new release on push to master branch

#### Flags

- `--no-npm` - Turn off `@semantic-release/npm` in `.releaserc` and remove `NPM_TOKEN` secret from `release.yml`

**NOTE:** If it detects a `bsconfig.json` meaning it's a [ReScript](http://rescript-lang.org/) project, it will remove the linting and replaces it with `npm run build` which is needed to run the tests.

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
