# egui tauri template

## Description

This is a template for creating an [egui](https://github.com/emilk/egui) app that runs inside a [tauri](https://github.com/tauri-apps/tauri) webview window.

This is a reduced version of [eframe_template](https://github.com/emilk/eframe_template) with tauri and [hframe](https://github.com/noxware/hframe) added.

## Why I would use egui inside a webview instead of a native window?

Although this adds a lot of overhead, it can be useful if you need to show web content alongside your egui app or maybe
use some web API.

For example, you can use this to embed multimedia content that the web naturally supports like videos.
Or maybe you want to show an `iframe`.

This template comes with [hframe](https://github.com/noxware/hframe) to help you with that.

## Screenshot

![Screenshot](https://github.com/noxware/egui-tauri-template/assets/7684329/9c31d085-fd78-4af8-a0fb-ab75243f55a5)

## Usage as desktop app

Install Tauri CLI and run `cargo tauri dev`.

## Usage as web app

Install Trunk and run `trunk serve`.

> Note: When used as a web app, Tauri commands will not work.
