# MacroKeyB

<img src="https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/icon.png?raw=true" width="200" alt="MacroKeyB Logo">

MacroKeyB is a Linux-exclusive tool (currently) that transforms any secondary keyboard into a powerful macro controller. By intercepting and blocking input from a secondary keyboard, MacroKeyB allows you to assign custom Lua scripts to each key, creating a dedicated macro pad for productivity, gaming, or creative workflows. 

This is my first Tauri/Rust project, please leave a star ⭐, that shows me I can continue this project 😀

## Screenshots

![MacroKeyB Demo2](https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/example3.gif?raw=true)
![MacroKeyB Demo1](https://github.com/spinalcord/MacroKeyB/blob/main/ReadmeImages/example.jpg?raw=true)

## Installation/Download/Run
You need root privilges to run MacroKeyB. Read the "Requirements" below why we need that.
```sh
sudo -E myapp.AppImage
```
I recommend the use of `deb` file for Ubuntu LTS like distribution (https://github.com/spinalcord/MacroKeyB/releases).

in case you are using a really new **Nvidia** card or a really old **Nvidia** card right now, use the parameter below

```sh
WEBKIT_DISABLE_DMABUF_RENDERER=1 sudo -E myapp.AppImage
```

- Nvidia 20xx 30xx and 40xx should work fine.
- AMD Cards should work out of the box


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
- Your keyboard must include "keyboard" as name like "logitech g19 keyboard" (Working for a better solution)

### x11 (and Wayland)
I tested MacroKeyB with KDE Plasma x11 and Wayland.
- x11: everything should work
- Wayland: everything might work but sending keys depends on DE, for instance KDE Wayland worked for me.

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
function superCoolFunction()
  combo({"ctrl", "alt", "t"}, 100) -- Open Terminal with 100 Delay for combo
  wait(2000)
  tap("l") -- "l"
  wait(200)
  tap("s") -- "s"
  wait(200)
  tap("enter") -- enter  
end

superCoolFunction()
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

use the .deb file instead of AppImage if you facing this error
```
macrokeyb: symbol lookup error: /tmp/.mount_macrokrq23lr/usr/lib/libgcrypt.so.20: undefined symbol: gpgrt_add_post_log_func, version GPG_ERROR_1.0
```

## ?

Please feel free to report issue or give me feedback.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Tauri](https://tauri.app/) for the application framework
- [Svelte](https://svelte.dev/) for the UI components


