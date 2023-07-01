# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2023-07-01

### 🚀 Features

- *(app)* Support expiring notifications
- *(app)* Add logging via `tracing`
- *(app)* Support showing a startup notification
- *(cairo)* Show the notification summary
- *(command)* Support filtering by the notification message
- *(config)* Add a configuration file to specify geometry
- *(config)* Support configuring the window based on notification urgency
- *(config)* Support changing the format of the message
- *(config)* Allow setting a custom text for urgency
- *(config)* Allow running custom OS commands based on urgency
- *(config)* Support global locations for the config file
- *(config)* Embed the default configuration into binary
- *(config)* Switch to Tera for more robust template rendering
- *(config)* Make log verbosity configurable
- *(config)* Support having a default value for urgency text
- *(dbus)* Parse notifications from D-Bus messages
- *(dbus)* Register the notification listener as a server
- *(dbus)* Send close notification signal on button press
- *(dbus)* Designate an ID for notifications
- *(dbus)* Support showing history and closing notifications via dbus
- *(dbus)* Add reply messages to D-Bus methods
- *(notification)* Support showing the date of notifications
- *(notification)* Support auto-timeout based on estimated read time
- *(notification)* Show notification date as human-readable text
- *(render)* Use pango/cairo for rendering text
- *(x11)* Initialize X11 connection and create window
- *(x11)* Support handling multiple notifications
- *(x11)* Add logging to X11 functions
- *(x11)* Add `wrap_content` option

### 🐛 Bug Fixes

- *(config)* Update notification history logic
- *(config)* Use string instead of borrowed string while deserializing
- *(dbus)* Skip closing already read notifications
- *(manager)* Check for notification count before showing history
- *(notification)* Do not run custom command while showing history
- *(typo)* Fix typo in security policy

### 🚜 Refactor

- *(app)* Use D-Bus server implementation for handling notifications
- *(cd)* Remove GitHub release requirement from crates.io step
- *(clippy)* Apply clippy suggestions
- *(clippy)* Apply clippy suggestions
- *(config)* Remove unused lifetime parameter
- *(config)* Rename auto_timeout field to auto_clear
- *(dbus)* Extract notification types into a module
- *(deps)* Switch to `thiserror` for error handling
- *(error)* Update the type alias for standard Result type
- *(lib)* Switch to library layout
- *(notification)* Create a notification manager
- *(readme)* Use HTML badges
- *(template)* Skip serializing internal fields
- *(template)* Improve error handling while parsing/rendering

### 📚 Documentation

- *(github)* Add Code Of Conduct
- *(github)* Add pull request template
- *(github)* Add issue templates
- *(github)* Add contribution guidelines
- *(github)* Add security policy
- *(license)* License the project under Apache 2.0 or MIT license
- *(license)* Update license copyright years
- *(project)* Update emojis in the description
- *(readme)* Add README.md
- *(readme)* Add sections to README.md
- *(readme)* Fix center in README.md
- *(readme)* Add more sections to README.md
- *(readme)* Add demo GIFs to README.md
- *(readme)* Add usage information to README.md
- *(readme)* Add urgency configuration info to README.md
- *(readme)* Update minimum supported Rust version to 1.64.0
- *(readme)* Add ctl usage example
- *(readme)* Update logo link
- *(readme)* Add AUR instructions
- *(readme)* Add badges
- *(readme)* Update installation instructions for Arch Linux
- *(readme)* Add instructions for installing on Alpine Linux
- *(x11)* Fix typo in comment- *(no category)* Clear runst changelog


### 🎨 Styling

- *(assets)* Update demo recordings
- *(assets)* Update project logo
- *(dbus)* Format the introspection data
- *(notification)* Show the unread notification count
- *(notification)* Update the startup message
- *(readme)* Use HTML for the project header
- *(readme)* Update the emoji in README.md- *(no category)* Resolve some clippy warnings


### ⚙️ Miscellaneous Tasks

- *(assets)* Remove unnecessary asset
- *(bors)* Add bors config
- *(bors)* Remove custom timeout
- *(cargo)* Add project metadata
- *(cargo)* Add cargo profile configuration to manifest
- *(cargo)* Add project metadata
- *(cd)* Set up continuous deployment workflow
- *(ci)* Add continuous integration workflow
- *(ci)* Bump actions/checkout action to v3
- *(ci)* Switch to Swatinem/rust-cache action for caching
- *(ci)* Run CI for bors
- *(config)* Update example config
- *(config)* Show elapsed notification time after 1 minute
- *(config)* Rename `format` config value to `template`
  - **BREAKING**: rename `format` config value to `template`
- *(config)* Update default config
- *(deps)* Bump dependencies
- *(deps)* Bump dependencies
- *(deps)* Bump pango from 0.16.3 to 0.16.5 ([#12](https://github.com/ellsclytn/nofi/issues/12))
- *(deps)* Bump serde from 1.0.148 to 1.0.150 ([#13](https://github.com/ellsclytn/nofi/issues/13))
- *(deps)* Bump serde_json from 1.0.89 to 1.0.91
- *(deps)* Bump thiserror from 1.0.37 to 1.0.38
- *(deps)* Bump toml from 0.5.9 to 0.5.10
- *(deps)* Bump cairo-rs from 0.16.3 to 0.16.7
- *(deps)* Bump serde from 1.0.150 to 1.0.151
- *(deps)* Bump serde from 1.0.151 to 1.0.152
- *(deps)* Bump x11rb from 0.11.0 to 0.11.1
- *(deps)* Bump dbus from 0.9.6 to 0.9.7
- *(deps)* Bump dbus-crossroads from 0.5.1 to 0.5.2
- *(deps)* Bump regex from 1.7.0 to 1.7.1
- *(deps)* Bump toml from 0.5.10 to 0.5.11
- *(deps)* Bump colorsys from 0.6.6 to 0.6.7
- *(deps)* Bump toml from 0.5.11 to 0.7.0
- *(deps)* Bump toml from 0.7.0 to 0.7.1
- *(deps)* Bump serde_json from 1.0.91 to 1.0.92
- *(deps)* Bump toml from 0.7.1 to 0.7.2
- *(deps)* Bump serde_json from 1.0.92 to 1.0.93
- *(deps)* Bump GTK dependencies
- *(deps)* Bump transitive dependencies
- *(deps)* Bump thiserror from 1.0.38 to 1.0.39
- *(deps)* Bump serde_json from 1.0.93 to 1.0.94
- *(deps)* Bump rust-embed from 6.4.2 to 6.6.0 ([#36](https://github.com/ellsclytn/nofi/issues/36))
- *(deps)* Bump transitive dependencies
- *(deps)* Bump serde from 1.0.152 to 1.0.155
- *(deps)* Bump tera from 1.17.1 to 1.18.0
- *(deps)* Bump transitive dependencies
- *(deps)* Bump toml from 0.7.2 to 0.7.3
- *(deps)* Bump serde from 1.0.155 to 1.0.156
- *(deps)* Bump pango from 0.17.0 to 0.17.4
- *(deps)* Bump tera from 1.18.0 to 1.18.1
- *(deps)* Bump thiserror from 1.0.39 to 1.0.40
- *(deps)* Bump dirs from 4.0.0 to 5.0.0
- *(deps)* Bump serde from 1.0.156 to 1.0.157
- *(deps)* Bump serde from 1.0.157 to 1.0.158
- *(deps)* Bump regex from 1.7.1 to 1.7.2
- *(deps)* Bump regex from 1.7.2 to 1.7.3
- *(deps)* Bump rust-embed from 6.6.0 to 6.6.1
- *(deps)* Bump serde_json from 1.0.94 to 1.0.95
- *(deps)* Bump serde from 1.0.158 to 1.0.159
- *(deps)* Bump serde from 1.0.159 to 1.0.160
- *(deps)* Bump serde_json from 1.0.95 to 1.0.96
- *(deps)* Bump regex from 1.7.3 to 1.8.0
- *(deps)* Bump tracing-subscriber from 0.3.16 to 0.3.17
- *(deps)* Bump regex from 1.8.0 to 1.8.1
- *(deps)* Bump tracing from 0.1.37 to 0.1.38
- *(deps)* Bump dirs from 5.0.0 to 5.0.1
- *(deps)* Bump serde from 1.0.160 to 1.0.162
- *(deps)* Bump serde from 1.0.162 to 1.0.163
- *(deps)* Bump toml from 0.7.3 to 0.7.4
- *(deps)* Bump sscanf from 0.4.0 to 0.4.1
- *(deps)* Bump regex from 1.8.1 to 1.8.2
- *(deps)* Bump regex from 1.8.2 to 1.8.3
- *(deps)* Bump tera from 1.18.1 to 1.19.0
- *(deps)* Bump regex from 1.8.3 to 1.8.4
- *(deps)* Bump serde from 1.0.163 to 1.0.164
- *(deps)* Bump rust-embed from 6.6.1 to 6.7.0
- *(deps)* Bump GTK dependencies
- *(deps)* Downgrade tracing crate
- *(deps)* Bump x11rb from 0.11.1 to 0.12.0
- *(deps)* Bump transitive dependencies
- *(deps)* Bump serde_json from 1.0.96 to 1.0.97
- *(deps)* Bump toml from 0.7.4 to 0.7.5
- *(deps)* Bump serde_json from 1.0.97 to 1.0.99
- *(editorconfig)* Add EditorConfig configuration
- *(git)* Update .gitignore
- *(github)* Add Dependabot config
- *(github)* Add CODEOWNERS
- *(github)* Add Jekyll theme
- *(github)* Remove Jekyll theme
- *(github)* Enable sponsorships
- *(github)* Add custom domain for GitHub pages
- *(github)* Check dependabot updates daily
- *(release)* Add release instructions
- *(release)* Add notes about releasing
- *(release)* Update git-cliff config
- *(release)* Remove empty lines from the tag message

### Ci
- *(no category)* Prepare automated releases


<!-- generated by git-cliff -->
