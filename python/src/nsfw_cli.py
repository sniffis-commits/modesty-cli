"""Modesty CLI: thin command line over the nsfw_scanner library.

All scanning lives in the lib; this package only forwards argv.
Works installed (`pip install ./packages/nsfw-cli`, entry points
`modesty-nsfw-scan` and the compatibility alias `nsfw-scan`) or straight
from the checkout.
"""

import json
import os
import sys


def _bootstrap():
    try:
        import nsfw_scanner  # noqa: F401
        return
    except ImportError:
        pass
    here = os.path.dirname(os.path.abspath(__file__))
    candidate = os.path.normpath(os.path.join(here, "..", "..", "nsfw-lib", "src"))
    if os.path.isdir(candidate):
        sys.path.insert(0, candidate)


_bootstrap()

from nsfw_scanner import main as lib_main  # noqa: E402
from nsfw_scanner.scanner import (  # noqa: E402
    evaluate_image, get_sessions, get_face_session, get_binary_session,
    analyze_image_file,
)
from PIL import Image  # noqa: E402


def _cmd_scan_eval(argv):
    """scan-eval TARGET [--crops] — full model evaluation (no thresholds)."""
    import argparse
    parser = argparse.ArgumentParser(prog="scan-eval",
                                     description="Run every model without "
                                                 "thresholds, with optional "
                                                 "half/half crops for "
                                                 "low-quality images")
    parser.add_argument("target", help="Local image file to evaluate")
    parser.add_argument("--crops", action="store_true",
                        help="Also run half-vertical / half-horizontal crops "
                             "on low-quality images")
    parser.add_argument("--low-quality-dim", type=int, default=None,
                        help="Pixel dimension below which an image is "
                             "considered low quality (default: 1024, or "
                             "NSFW_LOW_QUALITY_MAX_DIM env var)")
    parser.add_argument("--early-stop", action="store_true",
                        help="Stop at the first crop that detects a female "
                             "(default: scan all crops)")
    args = parser.parse_args(argv)

    nsfw_sess, gender_sess = get_sessions()
    face_sess = get_face_session()
    binary_sess = get_binary_session()

    img = Image.open(args.target)
    result = evaluate_image(img, nsfw_session=nsfw_sess,
                            gender_session=gender_sess,
                            face_session=face_sess,
                            binary_session=binary_sess,
                            crops=args.crops,
                            fallback_url=args.target,
                            low_quality_max_dim=args.low_quality_dim,
                            early_stop=args.early_stop)
    print(json.dumps(result, indent=2, default=str))


def _cmd_scan_new(argv):
    """scan-new TARGET — newer rating function (analyze_pil_image)."""
    import argparse
    parser = argparse.ArgumentParser(prog="scan-new",
                                     description="Run the regular rating "
                                                 "function (analyze_pil_image) "
                                                 "on an image")
    parser.add_argument("target", help="Local image file to scan")
    parser.add_argument("--details", action="store_true",
                        help="Print rating plus nsfw/female scores and gender")
    parser.add_argument("--debug", action="store_true",
                        help="Explain the verdict (keyword hits, deciding stage)")
    parser.add_argument("--crops", action="store_true",
                        help="Run half-vertical / half-horizontal crops on "
                             "low-quality images, stop at first female "
                             "detection (early stop)")
    parser.add_argument("--low-quality-dim", type=int, default=None,
                        help="Pixel dimension below which an image is "
                             "considered low quality (default: 1024, or "
                             "NSFW_LOW_QUALITY_MAX_DIM env var)")
    parser.add_argument("--high-accuracy", action="store_true",
                        help="Run YOLOv8 localized body-part detection after "
                             "the fast scanner to resolve middle-tier "
                             "over-protection (revealing 6 vs risque 7 vs "
                             "explicit 10).  Costs ~570ms per image vs "
                             "~112ms for the fast path.")
    args = parser.parse_args(argv)

    nsfw_sess, gender_sess = get_sessions()
    face_sess = get_face_session()
    binary_sess = get_binary_session()

    img = Image.open(args.target)

    # With --crops, use evaluate_image with early_stop=True so it stops
    # at the first crop that detects a female.
    if args.crops:
        result = evaluate_image(img, nsfw_session=nsfw_sess,
                                gender_session=gender_sess,
                                face_session=face_sess,
                                binary_session=binary_sess,
                                crops=True,
                                fallback_url=args.target,
                                low_quality_max_dim=args.low_quality_dim,
                                early_stop=True)
    else:
        result = analyze_image_file(args.target, nsfw_sess, gender_sess,
                                    details=args.details, debug=args.debug,
                                    face_session=face_sess,
                                    binary_session=binary_sess,
                                    high_accuracy=args.high_accuracy)
    print(json.dumps(result, indent=2, default=str))


def main(argv=None):
    # Intercept new subcommands before forwarding to the lib
    if argv is None:
        argv = sys.argv[1:]
    if argv and argv[0] in ("scan-eval", "scan-new"):
        cmd = argv[0]
        rest = argv[1:]
        if cmd == "scan-eval":
            _cmd_scan_eval(rest)
        else:
            _cmd_scan_new(rest)
        return

    # Force verbose details for production debugging
    import argparse
    if argv is None:
        argv = sys.argv[1:]
    # If no --details, inject it before lib call
    has_details = '--details' in argv or '-d' in argv
    if not has_details:
        argv = argv + ['--details']
    return lib_main(argv)


if __name__ == "__main__":
    raise SystemExit(main() or 0)
