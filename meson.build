project(
  'protonup-gtk',
  'rust',
  version: '0.1.0',
  meson_version: '>= 0.59',
  license: 'GPL-3.0-or-later',
)

dependency('gtk4', version: '>= 4.12')
dependency('libadwaita-1', version: '>= 1.5')

cargo = find_program('cargo', required: true)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')

application_id = 'com.github.Mar0xy.ProtonUpGtk'

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)

subdir('data')
subdir('src')
