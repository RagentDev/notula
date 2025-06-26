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
- **Menu Bar**: File, Edit, and View menus with standard options
- **Status Bar**: Real-time display of line number, column number, and character count
- **Full-screen Text Editor**: Text editing area fills entire window space
- **Dynamic Window Title**: Shows "Untitled - Notepad" or "filename - Notepad" with "*" for modified files

### File Operations
- **New**: Create new document (clears current content)
- **Save**: Save current document to existing file path
- **Exit**: Close application
- File modification tracking with visual indicator in title

### Text Editing
- Multi-line text editing with full keyboard support
- Real-time cursor position tracking
- Character count display
- Automatic text change detection

### TODO/Placeholder Features
- Open file dialog (File → Open)
- Save As file dialog (File → Save As)
- Edit operations (Undo, Cut, Copy, Paste, Select All, Find)
- View options (Word Wrap, Font selection)

## Project Structure

```
src/
  main.rs    # Single file containing all application logic
Cargo.toml   # Project configuration and dependencies
README.md    # Basic project description
```