# Contributing Guidelines

*Pull requests, bug reports, and all other forms of contribution are welcomed and highly encouraged!* :octocat:

### Contents

- [Install Locally](#books-install-locally)
- [Another Tools](#wrench-another-tools)

> **This guide serves to set clear expectations for everyone involved with the project so that we can improve it
together while also creating a welcoming space for everyone to participate. Following these guidelines will help ensure
a positive experience for contributors and maintainers.**

## :books: Install Locally

Before you start contributing you must clone and installing this project on your local machine.

> Prerequisite: you must install [rust](https://www.rust-lang.org/tools/install)
> and [node](https://nodejs.org/en/download/package-manager/current) on your machine first

1. Clone the project

```sh
# using ssh
git clone git@github.com:oxwazz/github_contributions_widget.git
# or using https
git clone https://github.com/oxwazz/github_contributions_widget.git
```

2. [Create GitHub token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) &
   paste on `wrangler.toml`

```toml
[vars]
GITHUB_TOKEN = "" # <- replace this empty string with yout GitHub token
```

3. Open project and then run the project

```sh
# opening project
cd github_contributions_widget
# run
npx wrangler dev
```

done 🎉

## :wrench: Another Tools

While working this project i usually using this tools, like:

1. https://yqnn.github.io/svg-path-editor/ - create svg path
1. https://cloudconvert.com/ttf-to-woff2 - convert font .ttf format to .woff2
1. https://www.giftofspeed.com/base64-encoder/ - convert font .woff2 format to base64
1. https://inkscape.org/release - create wireframe