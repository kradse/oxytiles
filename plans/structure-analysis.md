# OxyTiles Structure Analysis & Recommendations

## Current Structure Assessment

### 📁 Current File Organization

```
oxytiles/
├── src/
│   ├── main.rs              # App entry point + OxyTiles struct
│   ├── camera.rs            # Empty placeholder
│   ├── canvas.rs            # Canvas combining TileMap + TileSet
│   ├── tile_map.rs          # TileMap with HashMap for tiles
│   ├── tile_set.rs          # TileSet with texture handle
│   └── ui/
│       ├── mod.rs           # UI orchestration
│       ├── editor.rs        # Central panel (canvas rendering)
│       └── side_panel.rs    # Right side panel (tileset picker)
```

---

## 🔴 Critical Issues Identified

### 1. **Tight Coupling & Mixed Concerns**

**Problem:**
```rust
// canvas.rs - Canvas owns BOTH domain data AND presentation state
pub struct Canvas {
    pub tile_size: Vec2,        // Presentation concern
    pub tile_map: TileMap,      // Domain data
    pub tile_set: TileSet,      // Domain data  
    pub selected_rect: Pos2,    // UI state
}
```

**Why This Is Problematic:**
- Canvas mixes rendering parameters (tile_size) with domain models (TileMap, TileSet)
- Hard to reuse TileMap/TileSet in other contexts (e.g., headless export, CLI)
- UI state (selected_rect) shouldn't be in a data structure
- Violates Single Responsibility Principle

---

### 2. **Confusing Data Models**

**Problem:**
```rust
// tile_map.rs
pub struct TileMap {
    pub size: Vec2,
    pub selected_rect: Option<Rect>,  // UI state in domain model!
    pub tiles: HashMap<(usize, usize), egui::Rect>,  // Rect used for UV coords
}
```

**Issues:**
- `HashMap<(usize, usize), Rect>` - Using `Rect` to store UV coordinates is confusing
- `selected_rect` is UI state, not part of the tile map data
- `egui::Rect` in domain model creates dependency on UI framework
- Sparse HashMap is fine, but unclear what Rect represents

**What It Should Be:**
```rust
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,  // Multi-layer support
}

pub struct Layer {
    pub name: String,
    pub tiles: HashMap<(u32, u32), TileInstance>,  // Clear naming
}

pub struct TileInstance {
    pub tileset_id: usize,  // Which tileset
    pub tile_id: u32,       // Which tile in that tileset
    pub flip_x: bool,
    pub flip_y: bool,
}
```

---

### 3. **Incomplete Camera System**

**Problem:**
```rust
// camera.rs
pub struct Camera;  // Empty!
```

**Missing:**
- Pan and zoom functionality
- Screen-to-world coordinate conversion
- Viewport bounds calculation
- Essential for proper editor UX

---

### 4. **UI State Mixed Everywhere**

**Selected Rect Appears 3 Times:**
1. `Canvas.selected_rect: Pos2` (screen position)
2. `TileMap.selected_rect: Option<Rect>` (UV coordinates)
3. Local UI logic in side_panel.rs

**Problem:** No clear source of truth for UI state

---

### 5. **No Clear Architectural Layers**

Current code lacks separation:
- **Domain Layer** (pure data: Project, TileMap, TileSet)
- **Editor Layer** (editor state, tools, camera)
- **UI Layer** (egui panels, rendering)
- **Infrastructure** (file I/O, serialization)

---

## ✅ Recommended Structure

### Philosophy: **Separation of Concerns**

Following Rust best practices and your DESIGN.md:

```
oxytiles/
├── Cargo.toml
├── DESIGN.md
├── plans/
│   └── structure-analysis.md
└── src/
    ├── main.rs                    # Entry point only
    │
    ├── app.rs                     # OxyTiles app state
    │
    ├── domain/                    # Pure data models (no egui deps)
    │   ├── mod.rs
    │   ├── project.rs             # Project root container
    │   ├── tile_map.rs            # TileMap data structure
    │   ├── tile_set.rs            # TileSet data structure
    │   └── layer.rs               # Layer data structure
    │
    ├── editor/                    # Editor-specific logic
    │   ├── mod.rs
    │   ├── state.rs               # EditorState (active tool, selection, etc.)
    │   ├── camera.rs              # Camera with pan/zoom
    │   ├── tools/
    │   │   ├── mod.rs             # Tool trait
    │   │   ├── pencil.rs          # Pencil tool
    │   │   ├── eraser.rs          # Eraser tool
    │   │   └── pan.rs             # Pan tool
    │   └── commands/              # Command pattern (future)
    │       ├── mod.rs
    │       └── place_tile.rs
    │
    ├── rendering/                 # Rendering logic
    │   ├── mod.rs
    │   └── canvas.rs              # Canvas rendering implementation
    │
    ├── io/                        # File operations (future)
    │   ├── mod.rs
    │   ├── project_file.rs        # Save/load .oxytiles
    │   └── export.rs              # Export to JSON
    │
    └── ui/                        # egui UI code
        ├── mod.rs                 # UI orchestrator
        ├── panels/
        │   ├── mod.rs
        │   ├── editor_panel.rs    # Central canvas panel
        │   ├── tileset_panel.rs   # Tileset picker panel
        │   ├── layers_panel.rs    # Layers panel (future)
        │   └── properties_panel.rs # Properties panel (future)
        └── menu_bar.rs            # Menu bar (future)
```

---

## 📊 Layer Responsibilities

### **Domain Layer** (`domain/`)
**Purpose:** Pure data structures, no UI dependencies

**Characteristics:**
- No `egui` imports
- Serializable with `serde`
- Can be used in CLI, tests, or other contexts
- Represents the "truth" of your project

**Example:**
```rust
// domain/tile_map.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMap {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: usize,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub tiles: HashMap<(u32, u32), TileInstance>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileInstance {
    pub tileset_id: usize,
    pub tile_id: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}
```

---

### **Editor Layer** (`editor/`)
**Purpose:** Editor-specific logic and state management

**Responsibilities:**
- Camera system (pan, zoom, coordinate conversion)
- Tool system (pencil, eraser, fill, etc.)
- Editor state (selected tile, active tool, active layer)
- Command pattern for undo/redo (future)

**Example:**
```rust
// editor/state.rs
pub struct EditorState {
    pub project: Project,           // Domain data
    pub camera: Camera,             // Viewport state
    pub active_tool: ToolType,      // Current tool
    pub selected_tile: Option<SelectedTile>,  // Which tile is selected
    pub active_layer_id: Option<usize>,       // Which layer is active
    pub show_grid: bool,            // UI toggles
}

#[derive(Debug, Clone, Copy)]
pub struct SelectedTile {
    pub tileset_id: usize,
    pub tile_id: u32,
}

// editor/camera.rs
pub struct Camera {
    pub position: Vec2,      // World position
    pub zoom: f32,           // Zoom level
    pub viewport_size: Vec2, // Screen size
}

impl Camera {
    pub fn screen_to_world(&self, screen_pos: Pos2) -> Pos2 { /* ... */ }
    pub fn world_to_screen(&self, world_pos: Pos2) -> Pos2 { /* ... */ }
    pub fn pan(&mut self, delta: Vec2) { /* ... */ }
    pub fn zoom_at(&mut self, point: Pos2, delta: f32) { /* ... */ }
}
```

---

### **Rendering Layer** (`rendering/`)
**Purpose:** Pure rendering logic, converts domain data to visuals

**Responsibilities:**
- Take domain data + camera state → render to screen
- Grid rendering
- Tile rendering with textures
- Selection overlays
- No state management (stateless rendering functions)

**Example:**
```rust
// rendering/canvas.rs
pub fn render_canvas(
    painter: &egui::Painter,
    tile_map: &TileMap,
    tilesets: &[TileSet],
    camera: &Camera,
    editor_state: &EditorState,
) {
    // 1. Calculate visible tiles based on camera
    let visible_bounds = camera.get_viewport_bounds();
    
    // 2. Render grid
    render_grid(painter, tile_map, camera);
    
    // 3. Render each layer
    for layer in &tile_map.layers {
        if layer.visible {
            render_layer(painter, layer, tilesets, camera);
        }
    }
    
    // 4. Render selection overlay
    if let Some(selected) = editor_state.selected_tile {
        render_selection_overlay(painter, selected, camera);
    }
}
```

---

### **UI Layer** (`ui/`)
**Purpose:** egui-specific interface code

**Responsibilities:**
- Panel layout and organization
- User input handling
- Calling rendering functions
- Updating editor state based on user actions

**Example:**
```rust
// ui/panels/editor_panel.rs
pub fn show(ctx: &egui::Context, app: &mut OxyTiles) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );
        
        // Handle input
        handle_camera_input(&response, &mut app.editor_state.camera);
        handle_tool_input(&response, &mut app.editor_state);
        
        // Render
        rendering::render_canvas(
            &painter,
            &app.editor_state.project.tile_map,
            &app.editor_state.project.tilesets,
            &app.editor_state.camera,
            &app.editor_state,
        );
    });
}
```

---

## 🎯 Key Improvements

### Improvement 1: **Clear Ownership Model**

**Before:**
```rust
// Canvas owns everything, unclear who should access what
pub struct Canvas {
    pub tile_map: TileMap,
    pub tile_set: TileSet,
    pub selected_rect: Pos2,
}
```

**After:**
```rust
// Clear hierarchy and single source of truth
pub struct OxyTiles {
    pub editor_state: EditorState,  // All editor state here
}

pub struct EditorState {
    pub project: Project,      // Domain data
    pub camera: Camera,        // Viewport state
    pub selected_tile: Option<SelectedTile>,  // UI state
    // ... other editor state
}

pub struct Project {
    pub name: String,
    pub tile_maps: Vec<TileMap>,     // Can have multiple maps
    pub tile_sets: Vec<TileSet>,     // Multiple tilesets
}
```

---

### Improvement 2: **Type Safety & Clarity**

**Before:**
```rust
pub tiles: HashMap<(usize, usize), egui::Rect>,  // What does Rect mean here?
```

**After:**
```rust
pub tiles: HashMap<(u32, u32), TileInstance>,

pub struct TileInstance {
    pub tileset_id: usize,  // Clear: which tileset
    pub tile_id: u32,       // Clear: which tile
    pub flip_x: bool,
    pub flip_y: bool,
}
```

**Benefits:**
- Self-documenting code
- Type system catches errors
- Easy to add rotation, opacity, etc.

---

### Improvement 3: **No Framework Dependencies in Domain**

**Before:**
```rust
// domain/tile_map.rs has egui dependency
pub tiles: HashMap<(usize, usize), egui::Rect>,
```

**After:**
```rust
// domain/tile_map.rs - no egui!
pub tiles: HashMap<(u32, u32), TileInstance>,

// Can serialize/deserialize without egui
#[derive(Serialize, Deserialize)]
pub struct TileMap { /* ... */ }
```

**Benefits:**
- Can use domain types in CLI tools
- Can export without egui context
- Easier to test
- Future-proof (can switch UI frameworks)

---

### Improvement 4: **Scalable to Features**

**Current structure makes it hard to add:**
- ✗ Multiple tilesets
- ✗ Multiple layers
- ✗ Undo/redo
- ✗ Save/load
- ✗ Different tools

**New structure supports:**
- ✓ Multiple tilesets (Project owns Vec<TileSet>)
- ✓ Multiple layers (TileMap owns Vec<Layer>)
- ✓ Undo/redo (Command pattern in editor/commands/)
- ✓ Save/load (Domain types are serializable)
- ✓ Tools (Tool trait in editor/tools/)

---

## 🔄 Migration Strategy

### Phase 1: Create Domain Layer ✅ Low Risk
1. Create `domain/` directory
2. Move and refactor data structures:
   - `tile_set.rs` → `domain/tile_set.rs` (remove egui deps)
   - `tile_map.rs` → `domain/tile_map.rs` (fix data model)
   - Create `domain/layer.rs`
   - Create `domain/project.rs`

### Phase 2: Extract Editor State ✅ Medium Risk
1. Create `editor/state.rs` with EditorState
2. Move UI state from Canvas to EditorState
3. Implement Camera properly in `editor/camera.rs`

### Phase 3: Refactor Rendering 🔄 Medium Risk
1. Create `rendering/canvas.rs` with pure rendering functions
2. Remove Canvas struct (replaced by rendering functions)
3. Update UI to call rendering functions

### Phase 4: Update UI Layer 🔄 Low Risk
1. Rename `ui/editor.rs` → `ui/panels/editor_panel.rs`
2. Rename `ui/side_panel.rs` → `ui/panels/tileset_panel.rs`
3. Update to use new EditorState and rendering

### Phase 5: Add Tool System 🔄 Optional
1. Create `editor/tools/mod.rs` with Tool trait
2. Implement basic tools (Pencil, Eraser)
3. Update UI to use tool system

---

## 📈 Comparison: Before vs After

| Aspect | Current Structure | Recommended Structure |
|--------|------------------|----------------------|
| **Domain Models** | Mixed with UI (egui::Rect) | Pure Rust, no UI deps |
| **State Management** | Scattered across Canvas, TileMap | Centralized in EditorState |
| **Camera** | Non-existent | Full implementation with pan/zoom |
| **Rendering** | Mixed in UI code | Separate rendering layer |
| **Extensibility** | Hard to add features | Easy to extend |
| **Testability** | Hard (requires egui context) | Easy (pure functions) |
| **Serialization** | Blocked by egui deps | Ready with serde |
| **Multi-layer Support** | Not possible | Built-in |
| **Multiple Tilesets** | Awkward (only one) | Natural (Vec<TileSet>) |
| **Tool System** | Hardcoded | Trait-based, extensible |

---

## 🎓 Rust Best Practices Applied

### 1. **Separation of Concerns**
- Domain, Editor, Rendering, UI are separate
- Each module has one clear responsibility

### 2. **Dependency Management**
- Domain layer has no UI dependencies
- UI layer depends on all others
- Clear dependency flow: Domain ← Editor ← Rendering ← UI

### 3. **Module Organization**
- Use of `mod.rs` for module exports
- Logical grouping by functionality
- Private implementation details hidden

### 4. **Type Safety**
- Strong types instead of generic tuples/Rects
- Newtype pattern where appropriate
- Self-documenting code

### 5. **Ownership & Borrowing**
- Clear ownership hierarchy (Project owns TileMap)
- Minimal cloning (use references in rendering)
- Mutable state isolated in EditorState

### 6. **Trait-Based Design**
- Tool trait for extensibility
- Serialization with serde traits
- Easy to add new implementations

---

## 🤔 Answering Your Question

### Is Your Current Structure "Ideal"?

**Short Answer:** No, but it's a great start! 🎉

**Why Not Ideal:**
1. ❌ Tight coupling between domain and UI
2. ❌ Unclear data model (egui::Rect for UV coords)
3. ❌ State scattered everywhere
4. ❌ Hard to extend with new features
5. ❌ Can't serialize/save projects
6. ❌ Missing camera implementation

**What You Did Well:**
1. ✅ Good module structure (ui/ directory)
2. ✅ Using Default trait appropriately
3. ✅ Clear separation of UI panels
4. ✅ Following Rust naming conventions
5. ✅ Good use of Option types

### Why the Recommended Structure is Superior

**1. Future-Proofing:**
- Easy to add save/load (domain is serializable)
- Easy to add undo/redo (command pattern ready)
- Easy to add new tools (trait-based)
- Easy to add CLI export (domain has no UI deps)

**2. Maintainability:**
- Clear where to put new code
- Easy to understand data flow
- Each file has one clear purpose

**3. Testability:**
- Can test domain logic without UI
- Can test rendering without full app
- Can test tools independently

**4. Performance:**
- Rendering can be optimized separately
- Only redraw when state changes
- Camera culling easier to implement

**5. Rust Idiomatic:**
- Follows community conventions
- Similar to other successful Rust projects
- Matches patterns in bevy_editor, egui_tiles, etc.

---

## 💡 Learning Points for Rust

### Common Beginner Patterns to Avoid:

1. **God Structs** (Canvas was becoming this)
   - ❌ One struct that owns everything
   - ✅ Break into logical units with clear responsibilities

2. **Framework Leakage**
   - ❌ egui types in domain models
   - ✅ Keep framework types at the edges (UI layer)

3. **UI State in Data Models**
   - ❌ selected_rect in TileMap
   - ✅ UI state in EditorState

4. **Flat Module Structure**
   - ❌ Everything in src/ root
   - ✅ Organized by responsibility (domain/, editor/, ui/)

### Rust Patterns You're Using Well:

1. ✅ **Module System** - Good use of mod.rs
2. ✅ **Traits** - Using Default
3. ✅ **Option Types** - For optional state
4. ✅ **Naming** - Following snake_case, clear names

---

## 🚀 Next Steps

### If You Want to Refactor:

1. **Read through this document** and ask questions
2. **Start with Phase 1** (Domain layer) - safest place to start
3. **Test after each phase** - make sure app still works
4. **Take it slow** - no need to rush, you're learning!

### If Current Structure Works for You:

You can keep it! But be aware:
- Adding features will get harder
- Technical debt will accumulate
- Might need bigger refactor later

### Recommended Approach:

**Start small:** Just fix the data model first
- Create proper `TileInstance` struct
- Remove egui deps from tile_map.rs
- Implement Camera

Then see how you feel and continue if you want!

---

## 📚 Resources

**Rust Module Organization:**
- [The Rust Book - Growing Projects](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

**Architecture Patterns:**
- Look at [bevy](https://github.com/bevyengine/bevy) for ECS patterns
- Look at [egui demo](https://github.com/emilk/egui/tree/master/crates/egui_demo_lib) for UI organization

**Similar Projects:**
- [bevy_editor](https://github.com/jakobhellermann/bevy_editor) - editor architecture
- [ldtk-rs](https://github.com/estivate/ldtk-rs) - level editor parsing

---

## ✨ Summary

Your current structure is a **solid MVP foundation**, but has **architectural issues** that will make growth painful. The recommended structure follows **Rust best practices** and **separation of concerns**, making it:

- ✅ Easier to extend
- ✅ Easier to test  
- ✅ Easier to maintain
- ✅ More idiomatic Rust
- ✅ Ready for save/load/export
- ✅ Framework-agnostic domain

The refactoring can be done **incrementally** without breaking your app, starting with the domain layer and working outward.
