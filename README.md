# dcm-dump

`dcm-dump` is a minimal Rust toolkit for inspecting DICOM files without any external dependencies.

Current capabilities:
- read the 128-byte preamble and four-byte `DICM` prefix
- detect the transfer syntax UID from the File Meta Information group

Near-term goals:
- parse individual data elements (starting with Explicit VR Little Endian)
- validate common tags against the DICOM dictionary
- surface human-readable previews for textual values and concise hex dumps for binary payloads
- flag malformed elements (unexpected lengths, missing delimiters, etc.)

Longer-term stretch ideas:
- graceful handling of sequences and undefined-length items
- configurable output formats (plain text, JSON, or CSV summaries)
- optional heuristics to spot anonymisation issues or missing required attributes

This crate is intended both as a simple CLI and as a reusable library module, so everything is kept dependency-free and straightforward to read.
