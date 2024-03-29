# RustLessPass

A password manager built in Rust, compatible with LessPass.

Powered by Yew for WebAssembly compatibility, it runs directly in your web browser, so there's no need for any server computation—only for delivering the app.

Inspired by LessPass, it follows the same principles but works independently.

This means you can generate strong passwords without relying on external servers.

## Screenshot

![imagen](https://github.com/M1n-74316D65/RustLessPass/assets/54779580/9edf60a7-581f-4474-beaf-20ba76841310)

## Usage

RustLessPass provides a simple and secure way to generate and manage passwords. Users only need to remember their master password, and RustLessPass will generate unique passwords for each site or service based on user-specific inputs.

To use RustLessPass:

1. Enter your master password.
2. Provide the website name and login information.
3. RustLessPass will generate a unique password for that site.

You can then use the generated password when signing up or logging into websites, without the need to store or remember individual passwords.

## Installation

To run RustLessPass locally, follow these steps:

1. Clone the repository:
```bash
git clone https://github.com/M1n-74316D65/RustLessPass.git
```
2. Navigate to the project directory:
```bash
cd RustLessPass
```
3. Install dependencies:
```bash
git submodule update --init --recursive
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```
4. Serve the application using the trunk development server:
```bash
trunk serve --open
```

This will open the RustLessPass application in your default web browser.

## Contributing

Contributions to RustLessPass are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## Special Thanks

Sincere gratitude to:

- [Pico CSS](https://picocss.com)
- [lesspass](https://github.com/lesspass/lesspass)
- [lesspass.rs](https://github.com/71/lesspass.rs)
- [Yew Framework](https://yew.rs)
- [Rust Programming Language](https://rust-lang.org)

## License

RustLessPass is licensed under the [GPL-3.0 License](LICENSE).
