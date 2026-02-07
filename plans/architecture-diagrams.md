# OxyTiles Architecture Diagrams

## Current Architecture (Your Implementation)

### Data Flow

```mermaid
graph TB
    subgraph "main.rs"
        App[OxyTiles App]
    end
    
    subgraph "Data Layer Mixed"
        Canvas[Canvas<br/>owns everything]
        TileMap[TileMap<br/>+ selected_rect UI state<br/>+ egui::Rect for tiles]
        TileSet[TileSet<br/>+ texture handle]
        Camera[Camera<br/>empty!]
    end
    
    subgraph "UI Layer"
        UIShow[ui::show]
        Editor[editor.rs<br/>renders + handles input]
        SidePanel[side_panel.rs<br/>tileset picker]
    end
    
    App --> Canvas
    Canvas --> TileMap
    Canvas --> TileSet
    Canvas -.x Camera
    
    UIShow --> Editor
    UIShow --> SidePanel
    Editor --> Canvas
    SidePanel --> Canvas
    
    style TileMap fill:#ff9999
    style Canvas fill:#ff9999
    style Camera fill:#ff9999
```

**Problems Highlighted:**
- 🔴 Canvas is a "god object" owning everything
- 🔴 TileMap contains UI state (selected_rect)
- 🔴 TileMap uses egui::Rect for data (framework leakage)
- 🔴 Camera not implemented
- 🔴 UI logic mixed with rendering

---

## Recommended Architecture

### Clean Layered Architecture

```mermaid
graph TB
    subgraph "Entry Point"
        Main[main.rs]
    end
    
    subgraph "Application Layer"
        App[OxyTiles App]
        EditorState[EditorState<br/>Owns all editor state]
    end
    
    subgraph "Domain Layer - Pure Data"
        Project[Project<br/>Container for all data]
        TileMap[TileMap<br/>Pure data, no UI]
        TileSet[TileSet<br/>Metadata only]
        Layer[Layer<br/>Multi-layer support]
    end
    
    subgraph "Editor Layer"
        Camera[Camera<br/>Pan, zoom, coordinates]
        Tools[Tool System<br/>Pencil, Eraser, etc]
        Commands[Commands<br/>Undo/Redo future]
    end
    
    subgraph "Rendering Layer"
        Renderer[Canvas Renderer<br/>Stateless functions]
    end
    
    subgraph "UI Layer"
        UIOrch[ui::show]
        EditorPanel[EditorPanel<br/>Central canvas]
        TilesetPanel[TilesetPanel<br/>Tileset picker]
        LayersPanel[LayersPanel<br/>future]
    end
    
    Main --> App
    App --> EditorState
    EditorState --> Project
    EditorState --> Camera
    EditorState --> Tools
    
    Project --> TileMap
    Project --> TileSet
    TileMap --> Layer
    
    UIOrch --> EditorPanel
    UIOrch --> TilesetPanel
    UIOrch --> LayersPanel
    
    EditorPanel --> Renderer
    Renderer --> TileMap
    Renderer --> TileSet
    Renderer --> Camera
    
    style Project fill:#99ff99
    style TileMap fill:#99ff99
    style TileSet fill:#99ff99
    style Layer fill:#99ff99
```

**Benefits:**
- ✅ Clear separation of concerns
- ✅ Domain layer has no UI dependencies
- ✅ Easy to test each layer independently
- ✅ Scalable architecture
- ✅ Follows Rust best practices

---

## State Management Comparison

### Current: Scattered State

```mermaid
graph LR
    subgraph "State is Everywhere!"
        A[Canvas.selected_rect<br/>position] 
        B[TileMap.selected_rect<br/>UV coords]
        C[side_panel.rs<br/>local state]
    end
    
    A -.? B
    B -.? C
    
    style A fill:#ffcccc
    style B fill:#ffcccc
    style C fill:#ffcccc
```

**Problem:** Three different places track selection, unclear which is the source of truth!

---

### Recommended: Centralized State

```mermaid
graph TB
    subgraph "EditorState - Single Source of Truth"
        ES[EditorState]
        ST[selected_tile:<br/>Option SelectedTile]
        AT[active_tool:<br/>ToolType]
        AL[active_layer_id:<br/>Option usize]
        SG[show_grid: bool]
    end
    
    ES --> ST
    ES --> AT
    ES --> AL
    ES --> SG
    
    UI1[EditorPanel] --> ES
    UI2[TilesetPanel] --> ES
    UI3[ToolsPanel] --> ES
    
    style ES fill:#99ff99
```

**Benefits:**
- ✅ One place to look for state
- ✅ Easy to serialize for save/load
- ✅ Clear ownership

---

## Module Dependencies

### Current: Circular Dependencies Risk

```mermaid
graph LR
    Main --> Canvas
    Canvas --> TileMap
    Canvas --> TileSet
    UI --> Canvas
    TileMap -.egui.-> UI
    
    style TileMap fill:#ffcccc
```

**Problem:** TileMap depends on egui, creating circular dependency risk

---

### Recommended: Clean Dependency Flow

```mermaid
graph LR
    subgraph "Layer 1: Domain"
        D[domain/*<br/>No dependencies]
    end
    
    subgraph "Layer 2: Editor"
        E[editor/*]
    end
    
    subgraph "Layer 3: Rendering"
        R[rendering/*]
    end
    
    subgraph "Layer 4: UI"
        U[ui/*]
    end
    
    D --> E
    E --> R
    R --> U
    D --> R
    D --> U
    
    style D fill:#99ff99
```

**Benefits:**
- ✅ One-way dependencies (no cycles)
- ✅ Domain can be reused in CLI, tests, etc.
- ✅ Easy to understand and maintain

---

## Data Model Comparison

### Current: Confusing Data Types

```rust
// What does Rect mean here? UV coords? Screen rect?
pub tiles: HashMap<(usize, usize), egui::Rect>

// UI state mixed with domain data
pub struct TileMap {
    pub size: Vec2,
    pub selected_rect: Option<Rect>,  // ❌
    pub tiles: HashMap<(usize, usize), egui::Rect>,  // ❌
}
```

---

### Recommended: Clear, Self-Documenting Types

```rust
// Clear what everything means
pub tiles: HashMap<(u32, u32), TileInstance>

pub struct TileMap {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub layers: Vec<Layer>,  // Multi-layer support
}

pub struct Layer {
    pub id: usize,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub tiles: HashMap<(u32, u32), TileInstance>,
}

pub struct TileInstance {
    pub tileset_id: usize,  // Which tileset
    pub tile_id: u32,       // Which tile in tileset
    pub flip_x: bool,
    pub flip_y: bool,
}

pub struct SelectedTile {
    pub tileset_id: usize,
    pub tile_id: u32,
}
```

---

## Feature Scalability

### Current Structure Limitations

```mermaid
graph TB
    subgraph "Hard to Add"
        F1[❌ Multiple Layers<br/>TileMap only has one grid]
        F2[❌ Multiple Tilesets<br/>Canvas has one TileSet]
        F3[❌ Save/Load<br/>egui types not serializable]
        F4[❌ Undo/Redo<br/>No command pattern]
        F5[❌ Different Tools<br/>Logic hardcoded in UI]
        F6[❌ Export JSON<br/>Domain tied to UI]
    end
    
    style F1 fill:#ffcccc
    style F2 fill:#ffcccc
    style F3 fill:#ffcccc
    style F4 fill:#ffcccc
    style F5 fill:#ffcccc
    style F6 fill:#ffcccc
```

---

### Recommended Structure: Built for Growth

```mermaid
graph TB
    subgraph "Easy to Add"
        F1[✅ Multiple Layers<br/>Layer is Vec in TileMap]
        F2[✅ Multiple Tilesets<br/>Vec TileSet in Project]
        F3[✅ Save/Load<br/>Domain is serde ready]
        F4[✅ Undo/Redo<br/>editor/commands/ ready]
        F5[✅ Different Tools<br/>Tool trait + implementations]
        F6[✅ Export JSON<br/>Domain has no UI deps]
        F7[✅ CLI/Headless<br/>Domain can run anywhere]
    end
    
    style F1 fill:#99ff99
    style F2 fill:#99ff99
    style F3 fill:#99ff99
    style F4 fill:#99ff99
    style F5 fill:#99ff99
    style F6 fill:#99ff99
    style F7 fill:#99ff99
```

---

## Tool System Architecture

### Recommended: Extensible Tool System

```mermaid
graph TB
    subgraph "Tool Trait"
        Tool[Tool Trait<br/>on_mouse_down<br/>on_mouse_move<br/>on_mouse_up]
    end
    
    subgraph "Tool Implementations"
        Pencil[Pencil Tool<br/>Place single tiles]
        Eraser[Eraser Tool<br/>Remove tiles]
        Fill[Fill Tool<br/>Flood fill]
        Select[Select Tool<br/>Rectangle selection]
        Pan[Pan Tool<br/>Move camera]
    end
    
    Tool --> Pencil
    Tool --> Eraser
    Tool --> Fill
    Tool --> Select
    Tool --> Pan
    
    EditorState[EditorState] --> Tool
    
    UI[UI Layer] --> EditorState
```

**Benefits:**
- Easy to add new tools
- Tools are testable independently
- Clean separation from UI code

---

## Rendering Pipeline

### Recommended: Stateless Rendering

```mermaid
sequenceDiagram
    participant UI as UI Panel
    participant Render as Renderer
    participant Camera as Camera
    participant Domain as Domain Data
    
    UI->>Render: render_canvas(painter, tilemap, camera, state)
    Render->>Camera: get_viewport_bounds()
    Camera-->>Render: visible bounds
    Render->>Render: render_grid()
    loop Each Layer
        Render->>Domain: get layer tiles
        Domain-->>Render: tile data
        Render->>Render: render_layer()
    end
    Render->>Render: render_selection_overlay()
    Render-->>UI: Done
```

**Benefits:**
- Pure functions (easy to test)
- No hidden state
- Rendering optimizations isolated
- Clear data flow

---

## Migration Path Visual

```mermaid
graph TB
    subgraph "Phase 1: Domain"
        P1A[Create domain/ directory]
        P1B[Move TileMap without egui]
        P1C[Move TileSet without egui]
        P1D[Create Layer struct]
        P1E[Create Project struct]
        
        P1A --> P1B --> P1C --> P1D --> P1E
    end
    
    subgraph "Phase 2: Editor State"
        P2A[Create EditorState]
        P2B[Move UI state to EditorState]
        P2C[Implement Camera]
        
        P1E --> P2A --> P2B --> P2C
    end
    
    subgraph "Phase 3: Rendering"
        P3A[Create rendering module]
        P3B[Extract rendering functions]
        P3C[Update panels to use renderer]
        
        P2C --> P3A --> P3B --> P3C
    end
    
    subgraph "Phase 4: UI Cleanup"
        P4A[Reorganize UI panels]
        P4B[Update imports]
        P4C[Test everything works]
        
        P3C --> P4A --> P4B --> P4C
    end
    
    style P1E fill:#99ff99
    style P2C fill:#99ff99
    style P3C fill:#99ff99
    style P4C fill:#99ff99
```

---

## Summary Comparison

| Aspect | Current 🔴 | Recommended ✅ |
|--------|-----------|----------------|
| **Architecture** | Flat, coupled | Layered, decoupled |
| **Data Models** | egui types mixed in | Pure Rust types |
| **State** | Scattered | Centralized in EditorState |
| **Camera** | Not implemented | Full implementation |
| **Tools** | Hardcoded | Trait-based system |
| **Layers** | Single grid only | Multi-layer support |
| **Tilesets** | One at a time | Multiple tilesets |
| **Serialization** | Blocked by egui | Ready with serde |
| **Testing** | Hard (needs egui) | Easy (pure functions) |
| **Extensibility** | Difficult | Easy to extend |

---

## Code Size Comparison

### Current Implementation
```
src/
├── main.rs              (~50 lines)
├── camera.rs            (1 line - empty!)
├── canvas.rs            (~40 lines)
├── tile_map.rs          (~25 lines)
├── tile_set.rs          (~18 lines)
└── ui/
    ├── mod.rs           (~15 lines)
    ├── editor.rs        (~55 lines)
    └── side_panel.rs    (~73 lines)

Total: ~277 lines
```

### After Refactoring (Estimated)
```
src/
├── main.rs              (~30 lines - simpler!)
├── app.rs               (~50 lines)
├── domain/
│   ├── mod.rs           (~20 lines)
│   ├── project.rs       (~50 lines)
│   ├── tile_map.rs      (~80 lines - more features!)
│   ├── tile_set.rs      (~60 lines)
│   └── layer.rs         (~40 lines)
├── editor/
│   ├── mod.rs           (~15 lines)
│   ├── state.rs         (~80 lines)
│   ├── camera.rs        (~100 lines - full impl!)
│   └── tools/
│       ├── mod.rs       (~40 lines)
│       ├── pencil.rs    (~30 lines)
│       └── eraser.rs    (~25 lines)
├── rendering/
│   ├── mod.rs           (~10 lines)
│   └── canvas.rs        (~120 lines)
└── ui/
    ├── mod.rs           (~20 lines)
    └── panels/
        ├── mod.rs       (~15 lines)
        ├── editor_panel.rs   (~60 lines)
        └── tileset_panel.rs  (~70 lines)

Total: ~915 lines
```

**More code, but:**
- ✅ Much more features (layers, camera, tools, etc.)
- ✅ Better organized and easier to understand
- ✅ Easier to maintain long-term
- ✅ Each file has single responsibility
- ✅ Ready for expansion (undo/redo, save/load, etc.)

**Lines per feature comparison:**
- Current: ~277 lines for basic functionality
- Recommended: ~915 lines for complete MVP with:
  - Full camera system
  - Multi-layer support
  - Multiple tilesets
  - Tool system
  - Better data models
  - Serialization ready
  
**That's roughly 3x the code for 10x the features!**
