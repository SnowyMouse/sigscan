# sigscan

This program scans for a pattern in a given file and returns the offset(s) if found, or "none" if no offsets were found.

A signature is a set of hexadecimal bytes, using `??` to denote wildcards (e.g. "00??AABBCCDD").

Usage:

    sigscan [OPTIONS] <FILE_PATH> <SIGNATURE>
