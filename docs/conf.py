# Configuration file for the Sphinx documentation builder.
# Synced from jbcom-control-center - customize as needed

import os
import sys

# Add source to path for autodoc
sys.path.insert(0, os.path.abspath("../src"))

# -- Project information -----------------------------------------------------
# TODO: Update these for your project
project = "PACKAGE_NAME"
copyright = "2025, Jon Bogaty"
author = "Jon Bogaty"

# Try to get version from pyproject.toml or package.json
try:
    import tomllib
    with open("../pyproject.toml", "rb") as f:
        data = tomllib.load(f)
        release = data.get("project", {}).get("version", "0.0.0")
except Exception:
    try:
        import json
        with open("../package.json") as f:
            release = json.load(f).get("version", "0.0.0")
    except Exception:
        release = "0.0.0"

# -- General configuration ---------------------------------------------------

extensions = [
    # Python documentation
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
    "sphinx_autodoc_typehints",
    # Markdown support
    "myst_parser",
    # Diagrams (optional - requires sphinxcontrib-mermaid)
    # "sphinxcontrib.mermaid",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# Source file suffixes
source_suffix = {
    ".rst": "restructuredtext",
    ".md": "markdown",
}

# -- Options for HTML output -------------------------------------------------

html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]
html_title = f"{project} Documentation"

html_theme_options = {
    "navigation_depth": 4,
    "collapse_navigation": False,
    "sticky_navigation": True,
    "includehidden": True,
    "titles_only": False,
}

# -- Extension configuration -------------------------------------------------

# autodoc settings
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

# autosummary settings
autosummary_generate = True

# napoleon settings (Google/NumPy style docstrings)
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = True
napoleon_use_param = True
napoleon_use_rtype = True

# intersphinx settings
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
}

# myst_parser settings
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "fieldlist",
    "tasklist",
]
myst_heading_anchors = 3
