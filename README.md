# egui tauri template

## Description

This is a template for creating an [egui](https://github.com/emilk/egui) app that runs inside a [tauri](https://github.com/tauri-apps/tauri) webview window.

This is basically a modified version of [eframe_template](https://github.com/emilk/eframe_template) with tauri added.

## Why I would use egui inside a webview instead of a native window?

Although this adds a lot of overhead, it can be useful if you need to show web content alongside your egui app or maybe
use some web API.

For example, you can use this to embed multimedia content that the web naturally supports like videos.
Or maybe you want to show an `iframe`.

## Screenshot

![Screenshot](https://github.com/noxware/egui-tauri-template/assets/7684329/5186f2a2-4a28-48b1-a531-1cd8ebdb640d)

## Usage

Install Tauri CLI and run `cargo tauri dev` to run the app.
