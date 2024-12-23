# GitHub Contributions Widget

[Latest Version]: https://img.shields.io/crates/v/package_manager_detector_rs.svg

[crates.io]: https://crates.io/crates/package_manager_detector_rs

[Rustc Version]: https://img.shields.io/badge/rustc-1.56+-lightgray.svg

[rustc]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

📷 Show your dynamically generated GitHub contributions to your GitHub profile or your website!

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

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have you! We have
a [contributing guide](./CONTRIBUTING.md) to help you get
involved in the project.

## License

[MIT](./LICENSE) License © 2024-PRESENT [Muhammad Rahmahalim](https://github.com/oxwazz)