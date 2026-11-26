# RisingVice measurements on 8 GB NVIDIA hardware

Hardware: AMD Ryzen 7 7700X, MSI NVIDIA GeForce RTX 3070, 8017 MB VRAM, 32 GB DDR5 RAM,
Windows 11. Console profile: Xbox Series X.
Game executable and package supplied by the user; nothing from the game is stored here.

## Frame rate by configuration

| Configuration | Frame rate | Notes |
|---|---|---|
| Fork default | 3.5 – 5.3 fps | |
| `RISSING_CARD_BUFFERS_MIN_MB=1` | **3.9 – 6.7 fps** | best steady result; keep this |
| + `RISSING_ASYNC_PRESENT=1` | peaks 10.6, dips 2.5 | spiky; peaks higher, floor lower |
| `RISSING_RELAY=2` | 30 fps at 3 – 13 draws/frame | stayed on menu screens; never reached the world |
| `RISSING_LATE_WAIT=3 RISSING_WIDE_BACK=1 RISSING_CARD_BUFFERS_MIN_MB=1 RISSING_CARD_BUFFERS_MB=1024` | 1.4 – 6.9 fps | worse and unstable on 8 GB |
| `RISSING_DISPLAY_MODE=1920x1080` | 3.1 – 4.1 fps in the world | no improvement over 4K: the cost is not fill rate |
| avatarmanz/Rissing-7900XTX fork | **0.4 – 1.7 fps** | needs ~35 GB RAM and 24 GB VRAM; both walls hit here |

## Where a world frame's time goes (fork default, ~200–280 ms)

* copying back into title memory: 90 – 290 ms, moving 25 – 96 MB
* on the GPU: 70 – 195 ms, often idle waiting
* preparing: 40 – 200 ms (buffer copies, texture uploads)
* GPU memory budget fully used: ~6.9 GB of ~7 GB

## Hypotheses tested and disproved

Recorded so they are not retried.

1. **The CPU copy-back scan is the cost.** Measured `BackScanUs` at about 1 ms per frame, zero
   differing pieces. Wrong: the scan is cheap.
2. **Scattered copy-back ranges are unmerged duplicates.** `card_dirty()` already calls `joined()`.
   The ~260 ranges per frame are genuinely scattered.
3. **Refused compute pipelines have empty bytecode.** A diagnostic logged bytecode length and
   nullness for every compute pipeline: 2 KB – 337 KB, never null. Wrong.
4. **Refused compute pipelines lack samplers.** Refused and working pipelines show identical
   sampler and parameter counts. Wrong.
5. **Lowering the display mode raises world fps.** 1920x1080 gave 3.1 – 4.1 fps, unchanged. Wrong.
6. **`RISSING_RELAY=2` helps.** It stayed on the menus. Wrong for world frames.

## The fix kept

`integer_format()` + `blend_for()` in `crates/Rissing-gpu/src/pipeline/host.rs`: blending is turned
off for integer render targets, which D3D12 refuses to blend into. Mesh pipeline refusals fell from
17 to 1, and textured geometry draws.

## Still open

* 7 compute pipelines refused with `E_INVALIDARG`, valid bytecode, no D3D12 message.
* Ray-traced lighting not implemented (the black areas).
* GPU memory exhausted on an 8 GB card.
