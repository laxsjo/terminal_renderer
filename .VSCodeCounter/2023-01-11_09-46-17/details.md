# Details

Date : 2023-01-11 09:46:17

Directory /home/rasmus/rust_projects/terminal_renderer

Total : 48 files,  6325 codes, 1583 comments, 1373 blanks, all 9281 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [Cargo.toml](/Cargo.toml) | TOML | 19 | 0 | 2 | 21 |
| [src/app.rs](/src/app.rs) | Rust | 0 | 19 | 5 | 24 |
| [src/fun.rs](/src/fun.rs) | Rust | 145 | 16 | 32 | 193 |
| [src/lib.rs](/src/lib.rs) | Rust | 118 | 59 | 44 | 221 |
| [src/main.rs](/src/main.rs) | Rust | 11 | 59 | 18 | 88 |
| [src/rasmus_lib/ansi_term/mod.rs](/src/rasmus_lib/ansi_term/mod.rs) | Rust | 8 | 0 | 3 | 11 |
| [src/rasmus_lib/ansi_term/private/cursor.rs](/src/rasmus_lib/ansi_term/private/cursor.rs) | Rust | 82 | 57 | 22 | 161 |
| [src/rasmus_lib/ansi_term/private/format.rs](/src/rasmus_lib/ansi_term/private/format.rs) | Rust | 114 | 0 | 17 | 131 |
| [src/rasmus_lib/ansi_term/private/line.rs](/src/rasmus_lib/ansi_term/private/line.rs) | Rust | 10 | 12 | 4 | 26 |
| [src/rasmus_lib/ansi_term/private/mod.rs](/src/rasmus_lib/ansi_term/private/mod.rs) | Rust | 21 | 5 | 6 | 32 |
| [src/rasmus_lib/ansi_term/private/screen.rs](/src/rasmus_lib/ansi_term/private/screen.rs) | Rust | 41 | 43 | 11 | 95 |
| [src/rasmus_lib/ansi_term_old.rs](/src/rasmus_lib/ansi_term_old.rs) | Rust | 211 | 0 | 15 | 226 |
| [src/rasmus_lib/events.rs](/src/rasmus_lib/events.rs) | Rust | 48 | 1 | 12 | 61 |
| [src/rasmus_lib/flags.rs](/src/rasmus_lib/flags.rs) | Rust | 47 | 0 | 13 | 60 |
| [src/rasmus_lib/input/mod.rs](/src/rasmus_lib/input/mod.rs) | Rust | 666 | 131 | 107 | 904 |
| [src/rasmus_lib/input/string_editor.rs](/src/rasmus_lib/input/string_editor.rs) | Rust | 166 | 13 | 31 | 210 |
| [src/rasmus_lib/linear_ui/implementations.rs](/src/rasmus_lib/linear_ui/implementations.rs) | Rust | 8 | 0 | 2 | 10 |
| [src/rasmus_lib/linear_ui/mod.rs](/src/rasmus_lib/linear_ui/mod.rs) | Rust | 273 | 34 | 73 | 380 |
| [src/rasmus_lib/linear_ui/widgets.rs](/src/rasmus_lib/linear_ui/widgets.rs) | Rust | 200 | 1 | 27 | 228 |
| [src/rasmus_lib/macros.rs](/src/rasmus_lib/macros.rs) | Rust | 37 | 8 | 3 | 48 |
| [src/rasmus_lib/math/general.rs](/src/rasmus_lib/math/general.rs) | Rust | 331 | 284 | 79 | 694 |
| [src/rasmus_lib/math/matrix.rs](/src/rasmus_lib/math/matrix.rs) | Rust | 372 | 287 | 60 | 719 |
| [src/rasmus_lib/math/mod.rs](/src/rasmus_lib/math/mod.rs) | Rust | 13 | 10 | 8 | 31 |
| [src/rasmus_lib/math/vec.rs](/src/rasmus_lib/math/vec.rs) | Rust | 385 | 0 | 55 | 440 |
| [src/rasmus_lib/math/vec/vec_ops.rs](/src/rasmus_lib/math/vec/vec_ops.rs) | Rust | 294 | 0 | 36 | 330 |
| [src/rasmus_lib/mod.rs](/src/rasmus_lib/mod.rs) | Rust | 11 | 0 | 1 | 12 |
| [src/rasmus_lib/referenceable_vec.rs](/src/rasmus_lib/referenceable_vec.rs) | Rust | 35 | 27 | 10 | 72 |
| [src/rasmus_lib/ui/app.rs](/src/rasmus_lib/ui/app.rs) | Rust | 0 | 4 | 2 | 6 |
| [src/rasmus_lib/ui/gui.rs](/src/rasmus_lib/ui/gui.rs) | Rust | 79 | 11 | 29 | 119 |
| [src/rasmus_lib/ui/input_manager.rs](/src/rasmus_lib/ui/input_manager.rs) | Rust | 41 | 5 | 11 | 57 |
| [src/rasmus_lib/ui/mod.rs](/src/rasmus_lib/ui/mod.rs) | Rust | 69 | 27 | 17 | 113 |
| [src/rasmus_lib/ui/panel.rs](/src/rasmus_lib/ui/panel.rs) | Rust | 66 | 5 | 18 | 89 |
| [src/rasmus_lib/ui/render.rs](/src/rasmus_lib/ui/render.rs) | Rust | 31 | 13 | 13 | 57 |
| [src/rasmus_lib/utils.rs](/src/rasmus_lib/utils.rs) | Rust | 405 | 130 | 109 | 644 |
| [src/render_3d/buffer.rs](/src/render_3d/buffer.rs) | Rust | 62 | 10 | 17 | 89 |
| [src/render_3d/camera.rs](/src/render_3d/camera.rs) | Rust | 98 | 5 | 23 | 126 |
| [src/render_3d/color.rs](/src/render_3d/color.rs) | Rust | 172 | 27 | 43 | 242 |
| [src/render_3d/color/color_ops.rs](/src/render_3d/color/color_ops.rs) | Rust | 147 | 0 | 15 | 162 |
| [src/render_3d/drawers.rs](/src/render_3d/drawers.rs) | Rust | 181 | 99 | 79 | 359 |
| [src/render_3d/mesh_loader.rs](/src/render_3d/mesh_loader.rs) | Rust | 237 | 4 | 46 | 287 |
| [src/render_3d/mod.rs](/src/render_3d/mod.rs) | Rust | 248 | 8 | 46 | 302 |
| [src/render_3d/panel.rs](/src/render_3d/panel.rs) | Rust | 75 | 2 | 19 | 96 |
| [src/render_3d/quaternion.rs](/src/render_3d/quaternion.rs) | Rust | 314 | 78 | 75 | 467 |
| [src/render_3d/renderer.rs](/src/render_3d/renderer.rs) | Rust | 180 | 45 | 44 | 269 |
| [src/render_3d/scene.rs](/src/render_3d/scene.rs) | Rust | 58 | 0 | 15 | 73 |
| [src/render_3d/shader.rs](/src/render_3d/shader.rs) | Rust | 31 | 5 | 15 | 51 |
| [src/render_3d/transform.rs](/src/render_3d/transform.rs) | Rust | 109 | 16 | 32 | 157 |
| [src/test_data.rs](/src/test_data.rs) | Rust | 56 | 23 | 9 | 88 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)