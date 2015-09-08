Servo embedding
===============
Stuff I need implemented in Servo for embedding purposes.

Network
-------
The network hooks are required to implement ad-blocking, request policing and
smart referers.

On any request, before it's made, there should be a callback called with the
request object containing its origin (chrome or the page it's coming from) and
it should return an `Option<Request>` where `None` signifies the request should
not be made, and `Some(request)` contains the optionally modified request.

Script
------
The script hooks are required for interaction between the chrome and the
chrome-sub-pages (status bar, tab bar, input bar).

There should be a bidirectional channel (possibly type safe on the Rust side)
so that the various parts of the chrome can be directed.

For example from chrome to page, when the buffer inside a window changes, I
need to send the updated information to the window status bar, such as URL, TLS
status and whatnot.

From page to chrome, when the user starts inputting things in the input bar, I
need to talk back and forth to know completions and to receive the final input
and do something with it.

I also need a way to run user-scripts.

Style
-----
The style hooks are required to implement user-styles.

There should be a way to prepend a stylesheet to any page.

Compositor
----------
I need a way to know the final height of the page before rendering it,
basically to know how big the status bar would be based on the font and other
styling, same goes for the input bar when completion is started.
