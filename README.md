# GitHub Contributions Widget

[badge-workflow]: https://img.shields.io/github/actions/workflow/status/oxwazz/cors-bypass/release.yml

[link-workflow]: https://github.com/oxwazz/github_contributions_widget/actions/workflows/release.yml

[badge-twitter]: https://img.shields.io/twitter/follow/oxwazz

[link-twitter]: https://x.com/oxwazz

[![badge-workflow]][link-workflow]
[![badge-twitter]][link-twitter]

📷 Show your dynamically generated GitHub contributions to your GitHub profile or your website!

### Contents

- [Usage](#usage)
    - [Optional Query Params](#optional-query-params)
- [FAQ](#faq)
- [Why](#why)
- [Limitation](#limitation)
- [Contributing](#contributing)
- [Credit](#credit)
- [License](#license)

## Usage

Easily showcase your GitHub contributions to your profile! Follow the steps below:

1. Copy the code snippet below and paste it into your `README.md` file.
1. Replace `/oxwazz` with your GitHub username (case-insensitive).

```markdown
![GitHub Contributions Widget](https://github-contributions-widget.oxwazz.com/oxwazz)
```

**Output** <br/>
![GitHub Contributions Widget](./assets/snapshot/v0.1.3.svg)

Congratulation! 🎉 You are now showing your GitHub contributions to your profile!

### Optional Query Params

You can use additional params for customize your needs.

| Description                       | Query Params | Default Value | Possible Value                 |
|-----------------------------------|--------------|---------------|--------------------------------|
| filter by PR states               | ?states=     | -             | "MERGED" \| "OPEN" \| "CLOSED" |
| Custom widget title               | ?title=      | -             | string                         |
| Custom showing total contribution | ?show-max=   | 3             | number (min=1 max=10)          |
| Custom theme                      | ?theme=      | light         | "light" \| "dark"              |

and you can combine all the query params. 💪

## FAQ

**1. why the widget is not updating? that causing my latest contributions not listed**

> GitHub has image caching by default, this causing your latest contributions not reflect on this widget.
> to solve this issue you can manually update the image using query params like `?refresh-cache=1`
> or you can update `?refresh-cache=` automatically using GitHub Action cronjob
> with [probablykasper/readme-template-action](https://github.com/probablykasper/readme-template-action),
> you can [see the example](https://github.com/oxwazz/oxwazz) in my repository.

## Why

We create this widget to effortlessly display your GitHub contributions on your profile, highlighting your coding
activity
and accomplishments to potential collaborators, employers, or your community. It provides a visually appealing way to
showcase your commitment to open source and personal projects, enhancing your professional presence.

## Limitation

This code is deployed on a [Cloudflare worker](https://developers.cloudflare.com/workers/languages/rust/). As we are
utilizing the free tier, there
are [certain limitations](https://developers.cloudflare.com/workers/platform/limits/#worker-limits) associated with it.

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have you! We have a
[contributing guide](./CONTRIBUTING.md) to help you get involved in the project.

## Credit

"GitHub Contributions Widget" is currently being developed and maintained
by [Muhammad Rahmahalim](https://github.com/oxwazz).<br>
This project is inspired by [LeetCode-Stats-Card](https://github.com/JacobLinCool/LeetCode-Stats-Card), but focuses on
showcasing GitHub open-source contributions instead of LeetCode
progress. Thank you!

## License

[MIT](./LICENSE) License © 2024-PRESENT [Muhammad Rahmahalim](https://github.com/oxwazz)