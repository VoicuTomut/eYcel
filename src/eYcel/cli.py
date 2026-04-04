#!/usr/bin/env python3
"""eYcel CLI — encrypt, decrypt, and validate Excel files."""

import sys
import argparse
from pathlib import Path

from eYcel.encrypt import encrypt_excel
from eYcel.decrypt import decrypt_excel
from eYcel.yaml_handler import load_rules, validate_rules


# ---------------------------------------------------------------------------
# Sub-command handlers
# ---------------------------------------------------------------------------

def cmd_encrypt(args: argparse.Namespace) -> int:
    """Run the encrypt sub-command."""
    input_path = Path(args.input)
    if not input_path.exists():
        print(f"ERROR: Input file not found: {input_path}", file=sys.stderr)
        return 1
    if input_path.suffix.lower() not in (".xlsx", ".xls", ".xlsm"):
        print(f"ERROR: Input must be an Excel file (.xlsx/.xls/.xlsm): {input_path}", file=sys.stderr)
        return 1

    output_path = Path(args.output)
    if not output_path.parent.exists():
        print(f"ERROR: Output directory does not exist: {output_path.parent}", file=sys.stderr)
        return 1

    try:
        rules_path = encrypt_excel(
            input_path=str(input_path),
            output_path=str(output_path),
            rules=None,  # auto-detect; future: load from args.rules if provided
        )
        if not getattr(args, "quiet", False):
            print(f"✅ Encrypted  → {output_path}")
            print(f"📄 Rules      → {rules_path}")
        return 0
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


def cmd_decrypt(args: argparse.Namespace) -> int:
    """Run the decrypt sub-command."""
    input_path = Path(args.input)
    rules_path = Path(args.rules)
    output_path = Path(args.output)

    if not input_path.exists():
        print(f"ERROR: Encrypted file not found: {input_path}", file=sys.stderr)
        return 1
    if not rules_path.exists():
        print(f"ERROR: Rules file not found: {rules_path}", file=sys.stderr)
        return 1
    if not output_path.parent.exists():
        print(f"ERROR: Output directory does not exist: {output_path.parent}", file=sys.stderr)
        return 1

    try:
        decrypt_excel(
            encrypted_path=str(input_path),
            rules_path=str(rules_path),
            output_path=str(output_path),
        )
        if not getattr(args, "quiet", False):
            print(f"✅ Decrypted  → {output_path}")
        return 0
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


def cmd_validate(args: argparse.Namespace) -> int:
    """Run the validate sub-command."""
    rules_path = Path(args.rules)
    if not rules_path.exists():
        print(f"ERROR: Rules file not found: {rules_path}", file=sys.stderr)
        return 1

    try:
        rules = load_rules(str(rules_path))
        is_valid, errors = validate_rules(rules)
        if is_valid:
            if not getattr(args, "quiet", False):
                cols = rules.get("columns", {})
                meta = rules.get("metadata", {})
                print(f"✅ Rules file is valid")
                print(f"   Source file : {meta.get('original_filename', 'unknown')}")
                print(f"   Created     : {meta.get('timestamp', 'unknown')}")
                print(f"   Columns     : {len(cols)}")
                for col, cfg in cols.items():
                    print(f"   • {col}: {cfg.get('transform', '?')}")
            return 0
        else:
            print("❌ Rules file is INVALID:", file=sys.stderr)
            for err in errors:
                print(f"   • {err}", file=sys.stderr)
            return 1
    except Exception as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


# ---------------------------------------------------------------------------
# Argument parser
# ---------------------------------------------------------------------------

def build_parser() -> argparse.ArgumentParser:
    """Build and return the argument parser."""
    parser = argparse.ArgumentParser(
        prog="eYcel",
        description="Excel Data Anonymization & Encryption Tool",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  eYcel encrypt -i data.xlsx -o encrypted.xlsx
  eYcel decrypt -i encrypted.xlsx -r encrypted_rules.yaml -o restored.xlsx
  eYcel validate -r encrypted_rules.yaml
""",
    )
    parser.add_argument("--version", action="version", version="eYcel 0.2.0")
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose output")
    parser.add_argument("-q", "--quiet", action="store_true", help="Suppress output")

    subparsers = parser.add_subparsers(dest="command", metavar="COMMAND")
    subparsers.required = False

    # -- encrypt --
    ep = subparsers.add_parser("encrypt", help="Encrypt an Excel file")
    ep.add_argument("-i", "--input", required=True, metavar="FILE", help="Input Excel file (.xlsx)")
    ep.add_argument("-o", "--output", required=True, metavar="FILE", help="Output encrypted Excel file")
    ep.add_argument("-r", "--rules", metavar="FILE", help="Pre-existing rules file (batch mode)")

    # -- decrypt --
    dp = subparsers.add_parser("decrypt", help="Decrypt an encrypted Excel file")
    dp.add_argument("-i", "--input", required=True, metavar="FILE", help="Encrypted Excel file")
    dp.add_argument("-r", "--rules", required=True, metavar="FILE", help="Rules YAML file")
    dp.add_argument("-o", "--output", required=True, metavar="FILE", help="Output restored Excel file")

    # -- validate --
    vp = subparsers.add_parser("validate", help="Validate a rules YAML file")
    vp.add_argument("-r", "--rules", required=True, metavar="FILE", help="Rules file to validate")

    return parser


def main() -> None:
    """Main CLI entry point."""
    parser = build_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(0)

    dispatch = {
        "encrypt": cmd_encrypt,
        "decrypt": cmd_decrypt,
        "validate": cmd_validate,
    }
    sys.exit(dispatch[args.command](args))


if __name__ == "__main__":
    main()
