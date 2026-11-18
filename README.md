<!-- logo -->
<p align="center">
  <img src="https://modesty.eu.org/images/logos/modesty-cli.svg" alt="Modesty" width="96" height="96">
</p>

# modesty-cli

> Command-line scanner (Python + Rust)

`nsfw-scan` / `modesty-nsfw-scan` CLI over the library, plus the Rust CLI (`nsfw-cli-rust`, formerly `nsfw-scan-rust`) and the Native Messaging daemon used by the browser extensions.

## Details

| | |
|---|---|
| Language | Python + Rust |
| License | MIT |
| Website | [https://modesty.eu.org](https://modesty.eu.org) |
| Full repository map | [REPO_MAP.md](https://github.com/sniffis-commits/nfsw_scanner/blob/main/docs/REPO_MAP.md) |
| Status | Scaffold — code is being split from the private monorepo |

## Quick start

```sh
pip install modesty_nsfw_cli && nsfw-scan photo.jpg --details
```

## About Modesty

Modesty is a privacy-first NSFW content scanner. Scanning runs locally by
default; uploads are always opt-in. The public packages are split out of the
private monorepo into standalone repositories — see the
[repository map](https://github.com/sniffis-commits/nfsw_scanner/blob/main/docs/REPO_MAP.md) for the full local-to-GitHub naming.

Licensed under MIT. See the website's
[license review](https://modesty.eu.org/?p=licenses) for third-party data and model terms.
