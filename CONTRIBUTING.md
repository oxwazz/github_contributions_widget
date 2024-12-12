# Contributing Guidelines

*Pull requests, bug reports, and all other forms of contribution are welcomed and highly encouraged!* :octocat:

### Contents

- [Code of Conduct](#book-install-locally)

> **This guide serves to set clear expectations for everyone involved with the project so that we can improve it
together while also creating a welcoming space for everyone to participate. Following these guidelines will help ensure
a positive experience for contributors and maintainers.**

## :book: Install Locally

Before you start contributing you must clone and installing this project on your local machine.

> Prerequisite: you must install [rust](https://www.rust-lang.org/tools/install)
> and [node](https://nodejs.org/en/download/package-manager/current) on your machine first

1. clone the project

```sh
# using ssh
git clone git@github.com:oxwazz/github_contributions_widget.git
# or using https
git clone https://github.com/oxwazz/github_contributions_widget.git
```

2. [create GitHub token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens) &
   paste on `wrangler.toml`

```toml
[vars]
GITHUB_TOKEN = "" # <- replace this empty string with yout GitHub token
```

3. open project and then run the project

```sh
# opening project
cd github_contributions_widget
# run
npx wrangler dev
```

done 🎉
