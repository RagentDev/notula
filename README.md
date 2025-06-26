# Notula

A Windows Notepad-like text editor built with Rust and egui that supports **mixed text and image content**. Perfect for creating rich documents with embedded images while maintaining simple text file compatibility.

## Features

### 🖼️ **Image Support**
- **Paste images from clipboard** with `Ctrl+V`
- **Inline image display** with automatic scaling
- **Line-based editing** - images are placed on separate lines
- **Easy deletion** - backspace removes images when cursor is on image line

### 📝 **Text Editing**
- **Windows Notepad-like interface** with familiar menu structure
- **Line-by-line editing** with visual cursor indicator (→)
- **Real-time status bar** showing line number and character count
- **Full-screen text area** - no distracting sidebars

### 💾 **Smart File Format**
- **Save as `.txt` or `.md`** files with full compatibility
- **Dual-file system**: 
  - Text file contains `[img_load("id")]` placeholders for images
  - Metadata file (`.txt.meta` or `.md.meta`) stores base64-encoded image data
- **Perfect for version control** - text content can be tracked separately from binary images

### 🎯 **Easy to Use**
- **File Menu**: New, Save, Exit
- **Edit Menu**: Paste Image, Insert Sample Image, Delete Current Line
- **Keyboard shortcuts**: 
  - `Ctrl+V` - Paste image from clipboard
  - `Enter` - Create new line
  - `Backspace` - Delete current line (when empty) or image

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### Build from Source
```bash
git clone https://github.com/RagentDev/notula.git
cd notula
cargo build --release
cargo run
```

## Usage

### Basic Text Editing
1. Launch Notula
2. Type your text - each line is editable separately
3. Use `Enter` to create new lines
4. Use arrow or mouse to navigate between lines

### Adding Images
1. **From Clipboard**: Copy an image to your clipboard, then press `Ctrl+V`
2. **Sample Image**: Use `Edit → Insert Sample Image` to add a test gradient
3. **Menu Option**: Use `Edit → Paste Image (Ctrl+V)` 

### Saving and Loading
- **Save**: `File → Save` - saves both text and image metadata
- **Text file**: Contains your text with `[img_load("uuid")]` placeholders
- **Metadata file**: Contains base64-encoded image data in JSON format
- **Compatibility**: Text files remain readable in any text editor

### File Structure Example
**document.txt**:
```
This is my document with an image below:
[img_load("550e8400-e29b-41d4-a716-446655440000")]
More text after the image.
```

**document.txt.meta**:
```json
{
  "images": {
    "550e8400-e29b-41d4-a716-446655440000": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "data": "iVBORw0KGgoAAAANSUhEUgAA...",
      "width": 800,
      "height": 600
    }
  }
}
```

## Architecture

Built with modern Rust technologies:
- **[egui](https://github.com/emilk/egui)** - Immediate mode GUI framework
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)** - Native windowing
- **[image](https://github.com/image-rs/image)** - Image processing
- **[serde](https://serde.rs/)** - Serialization for metadata
- **[arboard](https://github.com/1Password/arboard)** - Clipboard access

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

See [CLAUDE.md](CLAUDE.md) for development guidance and architecture details.

## License

This project is open source. See the LICENSE file for details.

## Roadmap

- [ ] File dialogs for Open/Save As
- [ ] Text clipboard operations (Cut, Copy, Paste)
- [ ] Undo/Redo functionality
- [ ] Find/Replace
- [ ] Word wrap toggle
- [ ] Font customization
- [ ] Drag & drop image support
- [ ] Export to other formats (HTML, PDF)

---

*Notula - Simple notes with image support* 📝🖼️
