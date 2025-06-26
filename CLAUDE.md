# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Notula is a Windows Notepad-like application built with Rust using the egui framework. The application provides a full-featured text editor with proper menu bars, status bar, and file management capabilities that closely mimics the Windows Notepad experience.

## Development Commands

**Build the project:**
```bash
cargo build
```

**Run the application:**
```bash
cargo run
```

**Run in release mode:**
```bash
cargo run --release
```

**Build for release:**
```bash
cargo build --release
```

**Run tests:**
```bash
cargo test
```

**Check code without building:**
```bash
cargo check
```

**Format code:**
```bash
cargo fmt
```

**Run clippy linter:**
```bash
cargo clippy
```

## Architecture

- **Single-file application**: The entire application logic is contained in `src/main.rs`
- **GUI Framework**: Uses egui (0.24) via eframe for cross-platform desktop GUI
- **Application Structure**: 
  - `NotepadApp` struct holds the application state:
    - `text`: Current document content
    - `file_path`: Path to currently opened file (if any)
    - `is_modified`: Track if document has unsaved changes
    - `cursor_pos`: Current cursor position for status bar
  - Implements `eframe::App` trait for the main application loop
  - **Layout Components**:
    - `TopBottomPanel::top("menu_bar")`: File, Edit, View menus
    - `TopBottomPanel::bottom("status_bar")`: Line/column/character count
    - `CentralPanel`: Full-screen text editor
  - **File Operations**: New, Save, Save As functionality (Open requires file dialog implementation)

## Dependencies

- **egui 0.24**: Immediate mode GUI framework
- **eframe 0.24**: Framework for egui applications with native windowing
- **env_logger 0.11.8**: Logging framework
- **winapi 0.3**: Windows-specific APIs (Windows targets only)

## Current Features

### Windows Notepad-like Interface
- **Menu Bar**: File, Edit, and View menus with comprehensive options
- **Status Bar**: Real-time display of line number and character count
- **Full-screen Text Editor**: Line-by-line editing with visual cursor indicator (→)
- **Dynamic Window Title**: Shows "Untitled - Notepad" or "filename - Notepad" with "*" for modified files

### Image Support
- **Clipboard Paste**: Ctrl+V to paste images from clipboard
- **Menu Insert**: "Paste Image" and "Insert Sample Image" options
- **Inline Display**: Images rendered with automatic scaling to fit window
- **Line-based Layout**: Each image occupies its own line for easy organization
- **Easy Deletion**: Backspace removes images when cursor is on image line

### File Operations
- **Dual File Format**: 
  - Text files (.txt/.md) with `[img_load("id")]` placeholders
  - Metadata files (.txt.meta/.md.meta) with base64-encoded image data
- **Smart Save/Load**: Automatically handles text + metadata persistence
- **Version Control Friendly**: Text content separate from binary image data
- **Cross-platform Compatibility**: Standard text files readable anywhere

### Text Editing
- **Line-by-line editing**: Each line is a separate editable element
- **Navigation**: Arrow keys, mouse clicks, Enter for new lines
- **Real-time tracking**: Line numbers, character count, modification status
- **Mixed Content**: Seamless integration of text and images

### Data Architecture
- **ContentElement enum**: Handles both Text(String) and Image{id, width, height}
- **DocumentMetadata struct**: Serializable image storage with UUID references
- **Texture caching**: Efficient GPU texture management for image display
- **Base64 encoding**: Compact JSON storage of image data

### TODO/Placeholder Features
- Open/Save As file dialogs
- Text clipboard operations (Cut, Copy, Paste text)
- Edit operations (Undo, Redo, Select All, Find)
- View options (Word Wrap, Font selection)
- Drag & drop image support

## Project Structure

```
src/
  main.rs    # Single file containing all application logic
Cargo.toml   # Project configuration and dependencies
README.md    # Basic project description
```