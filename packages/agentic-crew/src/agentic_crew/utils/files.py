from __future__ import annotations

from pathlib import Path
from typing import Any, cast

import yaml


def load_config(path: Path | str) -> dict[str, Any]:
    """Loads a YAML configuration file.

    Args:
        path: The path to the YAML file.

    Returns:
        A dictionary containing the configuration.
    """
    with open(path) as f:
        loaded = yaml.safe_load(f) or {}
    return cast(dict[str, Any], loaded)
