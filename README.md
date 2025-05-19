# MacroKeyB

<img src="https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/icon.png?raw=true" width="200" alt="MacroKeyB Logo">

MacroKeyB is a Linux-exclusive tool (currently) that transforms any secondary keyboard into a powerful macro controller. By intercepting and blocking input from a secondary keyboard, MacroKeyB allows you to assign custom Lua scripts to each key, creating a dedicated macro pad for productivity, gaming, or creative workflows. 

Please leave a star ⭐, if you want, that shows me I can continue this project 😀

## Screenshots

![MacroKeyB Demo2](https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/example3.gif?raw=true)
![MacroKeyB Demo1](https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/example.jpg?raw=true)

## Installation/Download/Run

I recomment the use of AppImage (https://github.com/spinalcord/MacroKeyB/releases)


You need root privilges to run MacroKeyB. Read the "Requirements" below why we need that.
```sh
sudo -E myapp.AppImage
```

in case you are using a really new **Nvidia** card or a really old **Nvidia** card right now. 30xx and 40xx should work fine
```sh
WEBKIT_DISABLE_DMABUF_RENDERER=1 sudo -E myapp.AppImage
```



## Features

- **Multi-Keyboard Support**: Connect a secondary keyboard to your system while keeping your primary keyboard untouched
- **Input Blocking**: Completely intercepts input from the secondary keyboard so it doesn't interfere with your normal typing
- **Lua Scripting**: Program powerful macros using the Lua programming language
- **Auto-Detection**: Easily identify and configure your secondary input devices
- **Customizable**: Create, edit, and organize your macros with an intuitive interface
- **Systemwide Integration**: Runs in the background with a convenient system tray icon



## Requirements/Limitations

- Linux operating system
- Root privileges (required for keyboard input interception)
- Rust programming environment (for building from source)
- Secondary keyboard or input device

### Why Root Privileges Are Required
MacroKeyB requires root privileges for three essential functions:

1. **Reading Input Devices**: Access to `/dev/input/eventX` files is restricted to root users
2. **Blocking Keyboard Input**: The `EVIOCGRAB` ioctl call requires root privileges to capture keyboard input exclusively
3. **Modifying Device Behavior**: Setting non-blocking mode on input devices needs elevated permissions

## Usage

1. Launch MacroKeyB: `macrokeyb` or find it in your application menu
2. Click "Detect Input Device" to identify your secondary keyboard
3. Create a new macro by clicking "New"
4. Select the created macro and edit the Lua script in the editor
5. Click "Assign Key" and press a key on your secondary keyboard to link it to the macro
6. Save your configuration
7. MacroKeyB will continue running in the system tray

## Panic!
If somethings goes wrong just close MacroKeyB via system tray.

## Lua Scripting

MacroKeyB supports Lua scripting to create powerful and flexible macros. Here's a simple example:

```lua
-- Example macro that types "Hello World" and presses Enter
function run()
    keyboard.type("Hello World")
    keyboard.press("ENTER")
    return true
end
```

For more examples and the API documentation, see the [Wiki](https://github.com/yourusername/MacroKeyB/wiki).

## Key Features in Detail

### Secondary Keyboard Blocking

MacroKeyB completely blocks input from your secondary keyboard at the system level, ensuring key presses don't reach your active applications. This creates a dedicated device exclusively for executing your macros.

### Device Auto-Detection

Simply press a key on your secondary keyboard when prompted, and MacroKeyB will identify and configure it automatically.

### Lua Macro Engine

The built-in Lua interpreter provides access to:
- Keyboard simulation (typing, key presses, key combinations)
- Mouse control (movement, clicks, scrolling)
- System commands execution
- Application launching
- Text manipulation
- And more!

## Troubleshooting

- **Permission Issues**: MacroKeyB requires root privileges to intercept keyboard input. Run with `sudo` or ensure proper permissions are set.
- **Device Not Detected**: Make sure your secondary keyboard is properly connected and functioning.

## ?

Please feel free to report issue or give me feedback.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Tauri](https://tauri.app/) for the application framework
- [Svelte](https://svelte.dev/) for the UI components


