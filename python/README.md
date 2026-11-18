# Modesty - nsfw protection CLI

Command-line scanner over the Modesty `nsfw_scanner` Python library. Rates
websites, images, video/audio 1–10 for adult content.

## Install

```bash
pip install ./packages/nsfw-cli   # provides nsfw-scan, modesty-nsfw-scan
```

Or run straight from the checkout:

```bash
python packages/nsfw-cli/src/nsfw_cli.py https://example.com --details
python packages/nsfw-cli/src/nsfw_cli.py scan-new photo.jpg --details
```

## Scan an image

```bash
nsfw-scan photo.jpg --details           # rate a single image
nsfw-scan clip.mp4 --details            # rate a video (samples 1 fps)
nsfw-scan clip.mp4 --every-frame        # rate every frame (hunt hidden frames)
nsfw-scan "D:\Videos\My Clip.mp4" --db nsfw.sqlite   # match first, scan only unknown
```

## Optional hosted scan

The local scan stays local by default. To also upload a copy of a local media
file to the Modesty webservice, which performs an independent hosted scan and
stores its result:

```bash
nsfw-scan clip.mp4 --submit-to-webservice https://api.example.com
```

The URL is the webservice base URL; the CLI posts the file to `/v1/sample`.
The hosted result is included in the output as `webservice_submission`. This
uploads the media itself, not just the local rating. The current service
deletes its temporary upload after scanning but retains its hash, rating, and
scan metadata. Use only a service whose privacy terms you accept. This
submission option is separate from `--api-url`, which only checks for a
previously known hash/domain.

## Scan a website

```bash
nsfw-scan https://example.com --details
```

## Rating scale

| rating | meaning |
|--------|---------|
| `1` | no unsafe signal detected |
| `2` | synthetic content (render/screenshot/drawing) — not rated |
| `5` | women/girls detected; no stronger unsafe signal |
| `6` | romance — clothed, affectionate |
| `7` | revealing but not fully explicit |
| `10` | explicit adult content |

## Scan types

| scan type | when it is used |
|-----------|-----------------|
| `dns-badwords` | domain name matches a badword (no fetch) |
| `text` | page HTML text matched a keyword |
| `image` | image model classified the frame |
| `website` | whole-page verdict |

## License

MIT
