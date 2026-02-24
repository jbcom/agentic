from __future__ import annotations

from pathlib import Path

import yaml


def load_config(path: Path | str) -> dict:
    """Loads a YAML configuration file.

    Args:
        path: The path to the YAML file.

    Returns:
        A dictionary containing the configuration.
    """
    with open(path) as f:
        return yaml.safe_load(f)
