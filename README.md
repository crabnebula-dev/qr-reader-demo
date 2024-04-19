# QR Reader App

This app is a demo of a cross-platform (desktop + mobile) Tauri application that can read QR codes.

![demo image](./demo.png)

The parsed QR code value(s) will be written into the clipboard.

Check out the [release page](https://github.com/crabnebula-dev/qr-reader-demo/releases) to get started.

## Desktop

On desktop (Windows, Linux, MacOS), this is a **tray app** that allows the user to:

- **Scan** for QR codes on the entire screen (not reliable for larger resolution).
- **Crop** an area of the screen to search for a QR code.
- **Load** an image from the clipboard and parse a QR code from it.

We use [`screenshots`](https://github.com/nashaofu/xcap) and
[`bardecoder`](https://github.com/piderman314/bardecoder) crates
to scan and extract the QR code.

## Mobile

On mobile (iOS, Android) it is a simple app with only
one main window to **scan** barcodes via the camera.

We use [`tauri-plugin-barcode-scanner`](https://beta.tauri.app/features/barcode-scanner/),
which uses the native mobile functionality to scan a
QR code with the camera.

## Development

To start with building and development, please make sure you
followed the [prerequisites](https://beta.tauri.app/guides/prerequisites/)
installation guide from Tauri and all dependencies are
installed on your system.

If you want to build for mobile, you need to follow the install
steps for the
[mobile development preriquisites](https://beta.tauri.app/guides/prerequisites/#configure-for-mobile-targets).

This repository uses [`pnpm`](https://pnpm.io/) for the frontend packages,
so please make sure you have installed it as well.

Starting from `npm` you could install it via `npm install -g pnpm`.

1. Install repository dependencies
    ```sh
    pnpm install
    ```
2. Initialise the mobile development structure
    Android:
    ```sh
    pnpm tauri android init
    ```
    iOS:
    ```sh
    pnpm tauri ios init
    ```
3. Run the application in debug mode either local or in a mobile emulator.
    Local:
    ```sh
    pnpm tauri dev
    ```
    Android:
    ```sh
    pnpm tauri android dev
    ```
    iOS:

    ```sh
    pnpm tauri ios dev
    ```


