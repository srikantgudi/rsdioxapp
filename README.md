# rsdioxapp

## Sample app using Dioxus, a Rust Framework

### Features

- Signals - declare variables as signal
- Routing - define routes
- CLI to select different options while creating the app

### Project structure

```text
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

## Quick Start

```
git clone [your-repo]
cd rsdioxapp
dx serve
```

### Quick Demo

[Calendar](http://localhost:8090/calendar)

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve 
```

