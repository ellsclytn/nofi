## nofi

**A Rofi-driven notification manager**

<a href="https://github.com/ellsclytn/nofi/releases"><img src="https://img.shields.io/github/v/release/ellsclytn/nofi?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=GitHub&amp;logoColor=white" alt="GitHub Release"></a>
<a href="https://crates.io/crates/nofi/"><img src="https://img.shields.io/crates/v/nofi?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/ellsclytn/nofi/actions?query=workflow%3A%22Continuous+Integration%22"><img src="https://img.shields.io/github/actions/workflow/status/ellsclytn/nofi/ci.yml?branch=main&amp;style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=GitHub%20Actions&amp;logoColor=white" alt="Continuous Integration"></a>
<a href="https://github.com/ellsclytn/nofi/actions?query=workflow%3A%22Continuous+Deployment%22"><img src="https://img.shields.io/github/actions/workflow/status/ellsclytn/nofi/cd.yml?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=GitHub%20Actions&amp;logoColor=white&amp;label=deploy" alt="Continuous Deployment"></a>
<a href="https://docs.rs/nofi/"><img src="https://img.shields.io/docsrs/nofi?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Documentation"></a>

https://github.com/ellsclytn/nofi/assets/8725013/b3c5e53b-7ba9-44bd-a920-81a408b84cb9

`nofi` is a distraction-free notification center. While most notification daemons make immediate popups a key function, `nofi` is designed with such functionality as an anti-feature: notifications are intended to be viewed, but not to annoy. Notifications can be viewed at the user's discretion by launching `nofi`'s Rofi-driven notification manager.

`nofi` is a server implementation of [freedesktop.org](https://www.freedesktop.org/wiki) - [Desktop Notifications Specification](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html) and it can be used to receive notifications from applications via [D-Bus](https://www.freedesktop.org/wiki/Software/dbus/).

### The name?

A portmanteau of "[notification](https://wiki.archlinux.org/title/Desktop_notifications)" and [Rofi](https://github.com/davatorium/rofi).

## Features

- Template-powered ([Jinja2](http://jinja.pocoo.org/)/[Django](https://docs.djangoproject.com/en/3.1/topics/templates/)) notification text.
- Run custom OS commands based on the matched notifications.

## Installation

### From crates.io

`nofi` can be installed from [crates.io](https://crates.io/crates/nofi):

```sh
$ cargo install nofi
```

The minimum supported Rust version is `1.64.0`.

### Arch Linux

`nofi` can be installed from the [AUR](https://aur.archlinux.org/packages/nofi-bin) using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:

```sh
aura -A nofi-bin
```

### Binary releases

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/ellsclytn/nofi/releases).

### Build from source

#### Prerequisites

- [D-Bus](https://www.freedesktop.org/wiki/Software/dbus)

#### Instructions

1. Clone the repository.

```sh
$ git clone https://github.com/ellsclytn/nofi && cd nofi/
```

2. Build.

```sh
$ CARGO_TARGET_DIR=target cargo build --release
```

Binary will be located at `target/release/nofi`.

## Usage

### On Xorg startup

You can use [xinitrc](#xinitrc) or [xprofile](#xprofile) for autostarting `nofi`.

#### xinitrc

If you are starting Xorg manually with [xinit](https://www.x.org/archive/X11R6.8.0/doc/xinit.1.html), you can `nofi` on X server startup via [xinitrc](https://wiki.archlinux.org/title/Xinit#xinitrc):

`$HOME/.xinitrc`:

```sh
nofi &
```

Long-running programs such as notification daemons should be started before the window manager, so they should either fork themself or be run in the background via appending `&` sign. Otherwise, the script would halt and wait for each program to exit before executing the window manager or desktop environment.

In the case of `nofi` not being available since it's started at a faster manner than the window manager, you can add a delay as shown in the example below:

```sh
{ sleep 2; nofi; } &
```

#### xprofile

If you are using a [display manager](https://wiki.archlinux.org/title/Display_manager), you can utilize an [xprofile](https://wiki.archlinux.org/title/Xprofile) file which allows you to execute commands at the beginning of the X user session.

The xprofile file, which is `~/.xprofile` or `/etc/xprofile`, can be styled similarly to [xinitrc](#xinitrc).

#### As a D-Bus service

You can create a D-Bus service to launch `nofi` automatically on the first notification action. For example, you can create the following service configuration:

`/usr/share/dbus-1/services/org.ellsclytn.nofi.service`:

```ini
[D-BUS Service]
Name=org.freedesktop.Notifications
Exec=/usr/bin/nofi
```

Whenever an application sends a notification by sending a signal to `org.freedesktop.Notifications`, D-Bus activates `nofi`.

#### As a systemd service

`~/.config/systemd/user/nofi.service`:

```ini
[Unit]
Description=Nofi notification daemon
Documentation=man:nofi(1)
PartOf=graphical-session.target

[Service]
Type=dbus
BusName=org.freedesktop.Notifications
ExecStart=/usr/bin/nofi
```

You may then reload systemd and start/enable the service:

```sh
systemctl --user daemon-reload
systemctl --user start nofi.service
```

## Usage

`nofi` uses [`dbus-send(1)`](https://man.archlinux.org/man/dbus-send.1.en) to receive control instructions. There is currently only one instruction: viewing notification history.

```sh
# show the last notification
dbus-send --print-reply \
          --dest=org.freedesktop.Notifications \
          /org/freedesktop/Notifications/ctl \
          org.freedesktop.Notifications.History
```

An example use-case of this is to bind this to a key in your window manager, such as [i3](https://i3wm.org/):

```sh
bindsym $mod+grave exec dbus-send --print-reply \
        --dest=org.freedesktop.Notifications /org/freedesktop/Notifications/ctl org.freedesktop.Notifications.History
```

### Status Bar Integration

`nofi` broadcasts notification counts over a UNIX socket in the same format as [Rofication](https://github.com/DaveDavenport/Rofication). This means it can be integrated into status bars like [i3status-rust](https://github.com/greshake/i3status-rust/) via the [Rofication block](https://docs.rs/i3status-rs/latest/i3status_rs/blocks/rofication/index.html). The socket path follows the [XDG Base Directory](https://wiki.archlinux.org/title/XDG_Base_Directory) specification which usually exposes the socket at `/run/user/<UID>/nofi/socket`. This may vary between systems, so the socket path is output to `stdout` when `nofi` starts.

```ini
# Example i3status-rust integration

[[block]]
block = "rofication"
interval = 1
socket_path = "/run/user/1000/nofi/socket"
```

## Configuration

`nofi` configuration file supports [TOML](https://github.com/toml-lang/toml) format and the default configuration values can be found [here](./config/nofi.toml).

Configuration overrides can be placed in `$HOME/.config/nofi/nofi.toml`, or at a path of your choosing by specifying a `NOFI_CONFIG` environment variable.

### Global configuration

#### `log_verbosity`

Sets the [logging verbosity](https://docs.rs/log/latest/log/enum.Level.html). Possible values are `error`, `warn`, `info`, `debug` and `trace`.

#### `template`

Sets the template for the notification message. The syntax is based on [Jinja2](http://jinja.pocoo.org/) and [Django](https://docs.djangoproject.com/en/3.1/topics/templates/) templates.

Simply, there are 3 kinds of delimiters:

<!-- {% raw %} -->

- `{{` and `}}` for expressions
- `{%` or `{%-` and `%}` or `-%}` for statements
- `{#` and `#}` for comments

<!-- {% endraw %} -->

See [Tera documentation](https://tera.netlify.app/docs/#templates) for more information about [control structures](https://tera.netlify.app/docs/#control-structures), [built-in filters](https://tera.netlify.app/docs/#built-ins), etc.

##### Context

Context is the model that holds the required data for template rendering. The [JSON](https://en.wikipedia.org/wiki/JSON) format is used in the following example for the representation of a context.

```json
{
  "app_name": "nofi",
  "summary": "example",
  "body": "this is a notification 🦡",
  "urgency": "normal",
  "unread_count": 1,
  "timestamp": 1672426610
}
```

### Urgency configuration

There are 3 levels of urgency defined in the [Freedesktop](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html) specification and they define the importance of the notification.

1. `low`: e.g. "joe signed on"
2. `normal`: e.g. "you got mail"
3. `critical`: e.g. "your computer is on fire!"

You can configure `nofi` to act differently based on these urgency levels. For this, there need to be 3 different sections defined in the configuration file. Each of these sections has the following fields:

```toml
[urgency_{level}] # urgency_low, urgency_normal or urgency_critical
    custom_commands = []
```

#### `custom_commands`

With using this option, you can run custom OS commands based on urgency levels and the notification contents. The basic usage is the following:

```toml
custom_commands = [
    { command = 'echo "{{app_name}} {{summary}} {{body}}"' } # echoes the notification to stdout
]
```

As shown in the example above, you can specify an arbitrary command via `command` which is also processed through the template engine. This means that you can use the same [template context](#context).

The filtering is done by matching the fields in JSON via using `filter` along with the `command`. For example, if you want to play a custom notification sound for a certain application:

```toml
custom_commands = [
  { filter = '{ "app_name":"notify-send" }', command = 'aplay notification.wav' },
  { filter = '{ "app_name":"weechat" }', command = 'aplay irc.wav' }
]
```

The JSON filter can have the following fields:

- `app_name`: Name of the application that sends the notification.
- `summary`: Summary of the notification.
- `body`: Body of the notification.

Each of these fields is matched using regex and you can combine them as follows:

```toml
custom_commands = [
  { filter = '{ "app_name":"telegram|discord|.*chat$","body":"^hello.*" }', command = 'gotify push -t "{{app_name}}" "someone said hi!"' }
]
```

In this hypothetical example, we are sending a [Gotify](https://gotify.net/) notification when someone says hi to us in any chatting application matched by the regex.

## Related Projects

- [Rofication](https://github.com/DaveDavenport/Rofication)
- [runst](https://github.com/orhun/runst), which is what this project is a fork of.

## License

Licensed under either of [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [The MIT License](http://opensource.org/licenses/MIT) at your option.

## Copyright

Copyright © 2023, [Ellis Clayton](mailto:ellis@ellis.codes)
