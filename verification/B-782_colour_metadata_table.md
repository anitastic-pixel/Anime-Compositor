# B-782: a PNG's colour chunks named at import (D-902)

Written by `tests/b782_colour_metadata.rs` from `Fixtures/colour/metadata/expected_colour_metadata.json`. MEDIA_COLOR_METADATA_IGNORED is said for a chunk that is not sRGB; every picture is unchanged.

| Check | Result | Pass |
|---|---|---|
| chrm_rec2020.png: MEDIA_COLOR_METADATA_IGNORED, Found: cHRM white 31270 32900, red 70800 29200, green 17000 79700, blue 13100 4600. | MEDIA_COLOR_METADATA_IGNORED chrm_rec2020.png carries colour information this build does not read. Found: cHRM white 31270 32900, red 70800 29200, green 17000 79700, blue 13100 4600. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| chrm_rec2020.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| cicp_pq.png: MEDIA_COLOR_METADATA_IGNORED, Found: cICP 9/16/0/1. | MEDIA_COLOR_METADATA_IGNORED cicp_pq.png carries colour information this build does not read. Found: cICP 9/16/0/1. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| cicp_pq.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| cicp_srgb.png: no warning | no warning | yes |
| cicp_srgb.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| gama_45455.png: MEDIA_COLOR_METADATA_IGNORED, Found: gAMA 45455. | MEDIA_COLOR_METADATA_IGNORED gama_45455.png carries colour information this build does not read. Found: gAMA 45455. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| gama_45455.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| gama_linear.png: MEDIA_COLOR_METADATA_IGNORED, Found: gAMA 100000. | MEDIA_COLOR_METADATA_IGNORED gama_linear.png carries colour information this build does not read. Found: gAMA 100000. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| gama_linear.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| gama_srgb_chrm_2020.png: MEDIA_COLOR_METADATA_IGNORED, Found: gAMA 45455; cHRM white 31270 32900, red 70800 29200, green 17000 79700, blue 13100 4600. | MEDIA_COLOR_METADATA_IGNORED gama_srgb_chrm_2020.png carries colour information this build does not read. Found: gAMA 45455; cHRM white 31270 32900, red 70800 29200, green 17000 79700, blue 13100 4600. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| gama_srgb_chrm_2020.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| iccp.png: MEDIA_COLOR_METADATA_IGNORED, Found: iCCP 114 bytes. | MEDIA_COLOR_METADATA_IGNORED iccp.png carries colour information this build does not read. Found: iCCP 114 bytes. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| iccp.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| plain.png: no warning | no warning | yes |
| plain.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| srgb.png: no warning | no warning | yes |
| srgb.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| srgb_gama_chrm.png: no warning | no warning | yes |
| srgb_gama_chrm.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| srgb_with_iccp.png: MEDIA_COLOR_METADATA_IGNORED, Found: sRGB; iCCP 114 bytes. | MEDIA_COLOR_METADATA_IGNORED srgb_with_iccp.png carries colour information this build does not read. Found: sRGB; iCCP 114 bytes. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. | yes |
| srgb_with_iccp.png: decodes to the 16 stored bytes, the working picture plain.png's | byte for byte | yes |
| seq/: one warning, "gAMA 100000", drawings 1-3 | 3 drawings in frame_%04d.png carry colour information this build does not read. Found: gAMA 100000. The picture is drawn as sRGB, as every PNG is (document 21), with its numbers unchanged. Drawings: 1-3. | yes |
| two drawings of the sequence: still one warning, not two | warnings: 1 | yes |

24 of 24 pass.
