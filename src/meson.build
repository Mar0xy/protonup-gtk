cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: 'protonup-gtk',
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    cargo,
    'build',
    '--manifest-path', meson.project_source_root() / 'Cargo.toml',
    '--target-dir', meson.project_build_root() / 'target',
    '--release',
  ],
)
