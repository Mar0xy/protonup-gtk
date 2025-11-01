desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)

desktop_file = configure_file(
  input: 'com.github.Mar0xy.ProtonUpGtk.desktop.in',
  output: '@0@.desktop'.format(application_id),
  configuration: desktop_conf,
  install: true,
  install_dir: datadir / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_file = configure_file(
  input: 'com.github.Mar0xy.ProtonUpGtk.metainfo.xml.in',
  output: '@0@.metainfo.xml'.format(application_id),
  configuration: desktop_conf,
  install: true,
  install_dir: datadir / 'metainfo'
)

appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test('Validate appstream file', appstreamcli, args: ['validate', '--no-net', '--explain', appstream_file])
endif

install_data('com.github.Mar0xy.ProtonUpGtk.gschema.xml',
  install_dir: datadir / 'glib-2.0/schemas'
)

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file',
       compile_schemas,
       args: ['--strict', '--dry-run', meson.current_source_dir()])
endif
