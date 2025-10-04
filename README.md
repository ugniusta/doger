# Doger (Unfinished)
## A prototype web app for managing dog boarding

### Tech stack
- Dioxus (Frontend/backend framework written in Rust)
- PostgreSQL (Database)
- Tailwing (CSS styling)

### Features
- Timetable view 🗓️
- Add and delete dog stays to the timetable ➕
- Filter timetable date range ➡️ 
- Automatic timetable layout  🤖
- Ergonomic timetable date-range selection 👉
- Input data validation ✅

### Project structure
```
.
├── assets
│   ├── main.css
│   └── timetable.css
├── Cargo.lock
├── Cargo.toml
├── Dioxus.toml
├── input.css
├── src
│   ├── components
│   │   ├── bottom_menu.rs
│   │   ├── buttons.rs
│   │   ├── input_fields.rs
│   │   ├── modals.rs
│   │   ├── mod.rs
│   │   ├── pages
│   │   │   ├── home.rs
│   │   │   └── mod.rs
│   │   └── timetable
│   │       ├── mod.rs
│   │       ├── timeslot.rs
│   │       ├── timeslot_selection.rs
│   │       └── timetable.rs
│   ├── database.rs
│   ├── get_timeslot_index.js
│   ├── main.rs
│   ├── models.rs
│   └── server_fns.rs
└── tailwind.config.js
```
### Database structure

<img width="375" height="907" alt="image" src="https://github.com/user-attachments/assets/bd136d58-e25c-4923-aebe-9d30298c8b33" />

### TODO
- [ ] Probably totally rewrite the frontend in JS, because Dioxus is a pain in the ass to work with
- [ ] Ability to save dog and owner data 🐶
- [ ] Don’t use DB IDs in the user facing UI 🔢
- [ ] More rigorous validation and better errors 📐
- [ ] Authentication 🔒
- [ ] Caching, PWA/App, Offline support, etc… 🤷
