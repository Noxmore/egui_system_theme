This crate reads the current system theme and gives you an `egui` style to use in your program to hopefully make it look more cohesive with other apps for your user's platform.

# Platform support
### Linux
If the user is using KDE Plasma, it will read $HOME/.config/kdeglobals.

Otherwise, it will try to read the GTK4 or GTK3 theme via $HOME/.config/gtk-X.0/settings.ini.
This is more limited, as i had to partially write a css interpreter to get it working, if your theme doesn't work, make an issue!

### Windows
Uses [GetSysColor](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor).

NOTE: Some modern versions of Windows don't update this with the user's dark mode/accent color preferences for some reason, so if you are targeting windows, you should probably have an option to use the default `egui` theme.

### MacOS
Not supported yet, as i don't have access to any systems running it.
If you do, and want to help, a pull request would be greatly appreciated!