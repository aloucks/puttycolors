# VSCode Putty Color Theme

A PuTTY color theme based on the VSCode embedded terminal and the Atom One Dark default background color.

https://github.com/AlexAkulov/putty-color-themes

https://putty.org.ru/articles/putty-256-colors-and-themes.html

![image](vscode.png)

## Usage

1. Drag-and-drop `vscode.reg` onto `_puttycolor.js` in Windows Explorer for applying theme on _saved_ PuTTY sessions
    * Note that `_puttycolor.js` is `JScript`, _not_ `JavaScript`. If `.js` files are not associated with `JScript`,
      drag'n'drop will not work as expected
2. Or, double click on `vscode.reg` for applying theme to new PuTTY sessions

## AppV / Software Center

Windows machines using AppV (Software Center) may store the PuTTY settings in a different registry path. Drag and drop
`vscode.reg` onto `_puttycolor2.js` **while a PuTTY Configuration dialog is open** and wait for the confirmation.

Note that it performs a more exhaustive search of the registry and may take a while.
