# egui tauri template

## Description

This is a template for creating an egui app that runs inside a tauri webview window.

## Why I would use egui inside a webview instead of a native window?

Although this adds a lot of overhead, it can be useful if you need to show web content alongside your egui app or maybe
use some web API.

For example, you can use this to embed multimedia content that the web naturally supports like videos.
Or maybe you want to show an `iframe`.
