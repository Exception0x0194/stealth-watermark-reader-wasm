# Stealth Watermark Reader

A wasm pack for reading stealth watermark embedded in pictures' alpha channel, interpreted from [NovelAI's GitHub repo](https://github.com/NovelAI/novelai-image-metadata).

## Installation

```bash
npm install stealth-watermark-reader
```

## Usage

Below is an example of how to decode embedded information from an image with a specified path:

```javascript
import init, { decode_image_data } from 'stealth-watermark-reader';

// Initialize the WASM module only once
await init();

try {
    const response = await fetch('path/to/image.png');
    const imageBuffer = await response.arrayBuffer();
    const result = await decode_image_data(new Uint8Array(imageBuffer));
    console.log(result);
} catch (error) {
    console.error('Failed to decode watermark:', error);
}
```

### API Reference

- `decode_image_data(imageBytes: Uint8Array): string`
  - **Parameters**: 
    - `imageBytes` - A `Uint8Array` containing the bytes of the image.
  - **Returns**: A string containing the decoded information.
  - **Throws**: An error if the watermark cannot be decoded.

### Supported Image Formats

The function works for all formats of images with an alpha channel (e.g., `.png`, `.webp`), as long as Rust's image library can decode them.