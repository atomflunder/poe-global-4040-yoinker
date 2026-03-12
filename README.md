# poe-global-4040-yoinker

A CLI tool for Path of Exile `/global 4040` that automatically copies new invites to your clipboard.

## What is this?

In [Path of Exile](https://www.pathofexile.com/) there are many global chat rooms, `/global 4040` is used for sharing [challenge completions](https://www.poewiki.net/wiki/Challenge) to help players reach 40/40 challenges each league, earning some free microtransactions and a challenge totem pole along the way.

Invites for challenge completions are usually shared in the following format:
![Challenge completion format](./assets/format.png)

And people looking for an invite would have to manually type the invite into the in-game chat to get an invite from the host, for example `#+maven`.

This tool watches the `Client.txt` file for new invites, extracts the latest invite code, and copies it to your clipboard with a `#+` prefix.

You can then open the in-game chat and paste the invite code directly.

## Usage

1. Clone the repository and navigate to the project directory.
2. Replace the `FILE_PATH` constant in `src/main.rs` with the path to your `Client.txt` file.
3. Build the project using `cargo build --release`.
4. Run the compiled binary and wait for new invites to be copied to your clipboard.
5. Earn some challenge rewards!

![Challenge Totems](./assets/totems.png)

This tool was only tested on Linux with Wayland, but it should work on other platforms as well. If you encounter any issues, please open an issue.

## Disclaimer

> ⚠️ This tool should be used at your own risk! ⚠️

> The developers of Path of Exile, Grinding Gear Games, have had a fairly strict stance on automating interactions with Path of Exile. Players were banned in the past for example for using macros to automate flasks.  
> This is very much a grey area since this tool is not directly simulating keypresses (deliberately so), but it is still automating an interaction with the game.

Use this tool responsibly and be aware of the potential risks involved.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Tools used

- [notify](https://crates.io/crates/notify)
- [arboard](https://crates.io/crates/arboard)
- [regex](https://crates.io/crates/regex)

