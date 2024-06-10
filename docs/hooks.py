import extensions.typst_preview


def on_config(config, **kwargs):
    config.markdown_extensions.append(
        extensions.typst_preview.MyExtension(config, **kwargs)
    )
