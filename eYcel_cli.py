#!/usr/bin/env python3
"""eYcel CLI wrapper — delegates to the package CLI module."""
import sys
from pathlib import Path

# Allow running from source without installation
sys.path.insert(0, str(Path(__file__).resolve().parent / "src"))

from eYcel.cli import main

if __name__ == "__main__":
    main()
