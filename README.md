<p align="center"><img src="assets/logo.svg" alt="modesty-cli" width="110" height="110"></p>

# modesty-cli

> Command-line scanner

**Part of the [Modesty toolkit](https://github.com/sniffis-commits/modesty-meta)** — see the
[big README](https://github.com/sniffis-commits/modesty-meta) for the full picture, the rating scale and all repositories.

## What this is

The `modesty-nsfw-scan` / `nsfw-scan` command line tool over the library. Scan a URL, an image, a video or an audio file from a terminal and get a rating, a reason and the problem areas. Includes a Rust CLI and a Native Messaging daemon for browser integration.

## How to


```bash
pip install ./python
nsfw-scan photo.jpg --details
nsfw-scan clip.mp4 --skip-file --details      # write a .skp sidecar
nsfw-scan clip.mp4 --every-frame              # hunt hidden explicit frames
nsfw-scan https://example.com --details
```
- Optional hosted second opinion: `--submit-to-webservice <url>` uploads the
  file to a webservice `/v1/sample` (the media itself leaves your machine).
- `--api-url <url> --api-key <key>` only checks a hash against a hosted DB — no upload.
See `python/README.md` for the full flag list.


## Layout

| Path | Purpose |
|---|---|
| `python/` | source from the monorepo |
| `rust-cli/` | source from the monorepo |
| `rust-daemon/` | source from the monorepo |

## More

- Full overview and other repos: [modesty-meta](https://github.com/sniffis-commits/modesty-meta)
- Website: [https://modesty.cia.org.il](https://modesty.cia.org.il)
- Model weights: [modesty-models](https://github.com/sniffis-commits/modesty-models)
- Licence: MIT (see `LICENSE`). Third-party data/model terms are on the
  [site's licence page](https://modesty.cia.org.il/?p=licenses).
