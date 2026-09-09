# The dogfood screencast

What the GIF in `docs/readme-media/dogfood-candidate/` shows is a real run. The
provider turn inside it is a real, billed `claude` turn against a real account,
executed while the recorder was running. Nothing is replayed, re-enacted, or
reconstructed after the fact.

## What is in the recording

1. The enrolled provider runtime, and the digest the enrollment pins it to.
2. What the containment removes: network, host environment, writable tree.
3. The bridge running live, on a snapshot of Bullet Farm's own kernel source,
   with an elapsed clock and the contained process count visible throughout.
4. The Runner's journal from that run: the turn, the applied patch, the gate,
   the prepared Candidate, the preserved Candidate.
5. The Candidate itself, opened from the bundle that run preserved.
6. The receipt, with every eligibility flag false.

The recording also shows a real open defect rather than editing around it: the
Runner releases its lease before `bullet-gitd` cleans the workspace, and gitd's
cleanup re-reads that lease online, so a completed attempt ends on
`AUTHORITY_REFUSED`. The Candidate is already prepared and preserved by then.
It reproduces on the simulator too.

## Reproducing it

    media/dogfood/run-real-e2e.sh          # the real end-to-end run, alone
    media/dogfood/session.sh               # the narrated version, for recording

`run-real-e2e.sh` needs an operator-staged provider runtime, a dogfood policy,
binding and enrollment, and a credential grant. It refuses by name if any of
them is absent; it never falls back to the simulator.

To record and render:

    python3 scripts/lib/demo-gif-pty-record.py \
      --cast <dir>/session.cast --transcript <dir>/transcript.txt \
      --cols 120 --rows 34 --max-seconds 600 \
      -- bash media/dogfood/session.sh

    agg --theme "$THEME" --font-size 28 --line-height 1.35 \
        --fps-cap 10 --idle-time-limit 1.5 --renderer resvg \
        <dir>/session.cast dogfood-candidate-hi.gif

`THEME` is a pure-black background with the bright ANSI set, chosen so nothing
is dimmed:

    000000,ffffff,1c1c1c,ff5f5f,5fff5f,ffff5f,5fafff,ff5fff,5fffff,f0f0f0,
    808080,ff8787,87ff87,ffff87,87d7ff,ff87ff,87ffff,ffffff

The 1080-class variant is rendered natively at `--font-size 26` rather than
downscaled, so it carries no resampling loss.

## Quality, as measured rather than asserted

| Property | Measured |
| --- | --- |
| High-resolution GIF | 2049 x 1323, 5.3 MB |
| 1080-class GIF | 1903 x 1228, 4.8 MB |
| Frames | 108, 126.8 s |
| Colours per frame | 40 min, 255 max |
| Frames at the 256-colour ceiling | 0 |

No frame reaches the GIF palette ceiling, so no colour was dropped to fit it.
Decoding the GIF and decoding a lossless FFV1 master built from its frames give
the same pixels, `sha256 57486d88f0b0b33fae30631739d0ac48fc161316`, so the
distributed artifact carries the rendered frames exactly.

## What this is not

This is a `DOGFOOD_RUN`: an operational observation. It clears no release gate,
satisfies no self-hosting claim, and every eligibility flag in its receipt is
false. The recording says so on screen rather than in a footnote.

# The Control Tower screencast

`docs/readme-media/portal-real/portal-real.gif` walks the Control Tower against
a `bullet-farmd` serving the SQLite ledger that the dogfood run above wrote.
There are no fixtures and no mocked responses: the browser exchanges the
one-time bootstrap token against the running daemon, and every surface reports
`source bullet-kernel/sqlite-ledger` with the run's real sequence number.

It shows the real mission, the real work packages, and fence 1 from that run.
It also shows the Merge Rail reporting zero Candidates, which is the honest
state: `bullet-gitd` preserved the Candidate into a bundle, and nothing yet
publishes it into the ledger the Portal projects from. That gap is real and the
screenshot is not edited to hide it.

Capture it with `bullet-portal/ops/media/portal-capture.mjs`, run from the
portal package root so Playwright resolves:

    BULLET_BOOTSTRAP_TOKEN=<the one-time token farmd printed> \
    PORTAL_ORIGIN=http://127.0.0.1:4399 CAPTURE_DIR=<dir> \
      node ops/media/portal-capture.mjs

## Quality, as measured rather than asserted

| Property | Measured |
| --- | --- |
| Geometry | 1920 x 1200 |
| Size | 1.1 MB |
| Frames | 9, 20.8 s |
| Colours per frame | 249 to 254 |

GIF is an 8-bit format and a browser screenshot is 24-bit, so this one cannot
be lossless in the way the terminal recording is, and saying otherwise would be
the kind of claim this project exists to refuse. What was actually done:

- Chromium renders with `--disable-lcd-text`, `--disable-font-subpixel-positioning`
  and `--font-render-hinting=none`, which cuts the capture from 11,480 distinct
  colours to 2,115 without softening the glyphs.
- Each frame is quantized once to 256 colours with no dithering. Against the
  24-bit capture that moves at most 26/255 on 3.1% of pixels in the worst
  frame, and at most 7/255 on about 1% of pixels in six of the eight. Every
  changed pixel is a glyph antialiasing edge.
- The GIF then reproduces those quantized frames with a worst residual of
  1/255, so the distributed artifact is the quantized capture and nothing
  further was lost in encoding it.
