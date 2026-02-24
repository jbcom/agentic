# Configuration file for the Sphinx documentation builder.
# Outputs markdown via sphinx-markdown-builder for Astro Starlight consumption.

import os
import sys

# Add Python package sources to path for autodoc
sys.path.insert(0, os.path.abspath("../../packages/agentic-crew/src"))
sys.path.insert(0, os.path.abspath("../../packages/pytest-agentic-crew/src"))

# -- Project information -----------------------------------------------------
project = "Agentic Dev Library"
copyright = "2025, Jon Bogaty"
author = "Jon Bogaty"

try:
    import tomllib
    with open("../../pyproject.toml", "rb") as f:
        data = tomllib.load(f)
        release = data.get("project", {}).get("version", "0.0.0")
except Exception:
    release = "0.0.0"

# -- General configuration ---------------------------------------------------

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
    "sphinx_autodoc_typehints",
    "myst_parser",
    "sphinx_markdown_builder",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

source_suffix = {
    ".rst": "restructuredtext",
    ".md": "markdown",
}

# -- Options for HTML output -------------------------------------------------

html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]
html_title = f"{project} Documentation"

# -- Extension configuration -------------------------------------------------

autodoc_default_options = {
    "members": True,
    "member-order": "bysource",
    "special-members": "__init__",
    "undoc-members": True,
    "exclude-members": "__weakref__",
    "show-inheritance": True,
}
autodoc_typehints = "description"
autodoc_class_signature = "separated"

autosummary_generate = True

napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = True
napoleon_use_param = True
napoleon_use_rtype = True

intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
}

myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "fieldlist",
    "tasklist",
]
myst_heading_anchors = 3
