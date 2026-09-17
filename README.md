# Brief Summary
A streamlined, easy tool for testing if your GeoJSON, TileJSON, or template URL endpoints work. This is best for when you are working with tile servers and need to quickly verify that your data renders correctly.

# Minor Warning
Currently, the map instance may flicker or crash when loading endpoints that send large amounts of data (>~30MB). This performance bottleneck will be addressed in a future update.

# Tools Used
- [MapLibre GL](https://maplibre.org/)
- [Svelte](https://svelte.dev/)
- [Tauri](https://tauri.app/)