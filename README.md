miserve
=======
Servo based, vim-inspired, tinfoil hat approved, modal browser, that's what
this wants to be at least.

Features
--------
- [ ] vimperator inspired bindings
- [ ] vimperator inspired status bar
- [ ] vim inspired tab bar
- [ ] vim inspired buffers
- [ ] vim inspired windows
- [ ] vim inspired tabs
- [ ] built-in ad blocker
- [ ] built-in NoScript-like thingy
- [ ] built-in RequestPolicy-like thingy
- [ ] built-in GreaseMonkey support
- [ ] built-in Stylish-like support

Buffer
------
A buffer is a web page, when you close a window the page doesn't go away,
unless you close the buffer (which will also close the window).

This allows you to reopen a buffer in different window without losing its
state, or have it opened in multiple windows.

Window
------
A window is a container for a buffer, windows can be split vertically or
horizontally however many times you want, like in vim.

Tab
---
A tab is a container for windows.

Modes
-----
`NORMAL` mode will be like vimperator's normal mode.

`EDIT` mode will be enabled while inside a text input area, it will behave like
vim normal mode.

`INSERT` behaves like vim.

`REPLACE` behaves like vim.
