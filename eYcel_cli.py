#!/usr/bin/env python3
"""
eYcel Command Line Interface

Usage:
    eYcel encrypt --input <file> --output <file> [--rules <file>] [--quiet]
    eYcel decrypt --input <file> --rules <file> --output <file> [--quiet]
    eYcel validate --rules <file>
"""
import argparse
import sys
from pathlib import Path


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        prog='eYcel',
        description='Excel Data Anonymization & Encryption Tool'
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Encrypt command
    encrypt_parser = subparsers.add_parser('encrypt', help='Encrypt an Excel file')
    encrypt_parser.add_argument('-i', '--input', required=True, help='Input Excel file')
    encrypt_parser.add_argument('-o', '--output', required=True, help='Output encrypted file')
    encrypt_parser.add_argument('-r', '--rules', help='Rules file (batch mode)')
    
    # Decrypt command
    decrypt_parser = subparsers.add_parser('decrypt', help='Decrypt an encrypted file')
    decrypt_parser.add_argument('-i', '--input', required=True, help='Input encrypted file')
    decrypt_parser.add_argument('-r', '--rules', required=True, help='Rules file')
    decrypt_parser.add_argument('-o', '--output', required=True, help='Output restored file')
    
    # Validate command
    validate_parser = subparsers.add_parser('validate', help='Validate rules file')
    validate_parser.add_argument('-r', '--rules', required=True, help='Rules file to validate')
    
    # Global flags
    parser.add_argument('-v', '--verbose', action='store_true', help='Verbose output')
    parser.add_argument('-q', '--quiet', action='store_true', help='Suppress output')
    parser.add_argument('--version', action='version', version='%(prog)s 0.1.0')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    # TODO: Implement command dispatch
    if args.command == 'encrypt':
        print(f"Encrypt: {args.input} -> {args.output}")
        # TODO: Call encrypt_excel
    elif args.command == 'decrypt':
        print(f"Decrypt: {args.input} -> {args.output}")
        # TODO: Call decrypt_excel
    elif args.command == 'validate':
        print(f"Validate: {args.rules}")
        # TODO: Call validate_rules


if __name__ == '__main__':
    main()
