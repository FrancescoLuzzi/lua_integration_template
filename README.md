# lua_integration_template

template to create a simple project with:

- [x] lua integration
- [x] templating (minijinja, could also use others PRs are welcome)
- [ ] db using sqlx (integrated with lua via a ctx object or a global api)
- [ ] cache using bb8-redis (integrated with lua via a ctx object or a global api)
- [ ] axum integration:
  - [x] add routings via lua (using matchit)
  - [ ] route those new routes in axum
- [ ] cli integration (using argh, clap or custom)
- [ ] tui integration (using ratatui, see yazi for an example)
- [ ] gui integration (using winit, floem, gtk)
