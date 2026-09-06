# B-09, saving from the window

The window could open a project and not write one back. It can now, and this is what the writing does. Produced by `cargo test -p anime_compositor_app`, from `app/src/main.rs`.

Every row here is about one promise: **what comes back off the disk is the project that was open, including the parts this build does not understand.** The fixture is `Fixtures/projects/unknown_effect_project.json`, which names an effect no version of this build has. A window that saved only what it could model would drop that effect, the file would still open, the picture would still look right, and the loss would be found much later by the person who made the mask. That is the failure this table exists to catch.

| Check | Expected | Actual | Result |
|---|---|---|---|
| Save As says where the project went | Saved to <a temporary directory>\saved_elsewhere.json | Saved to <a temporary directory>\saved_elsewhere.json | pass |
| the file the person chose is now on disk | true | true | pass |
| the effect this build does not have is still in the saved file | true | true | pass |
| reopening the saved file and saving it again would write the same bytes | 2653 | 2653 | pass |
| and the same text, not merely the same length | true | true | pass |
| after Save As the window is showing the file that was written | <a temporary directory>\saved_elsewhere.json | <a temporary directory>\saved_elsewhere.json | pass |
| and calls it by its new name | saved_elsewhere.json | saved_elsewhere.json | pass |
| with no unsaved work outstanding | false | false | pass |
| Save writes to the file the project came from | Saved to <a temporary directory>\saved_elsewhere.json | Saved to <a temporary directory>\saved_elsewhere.json | pass |
| and writing it a second time changes nothing in it | {
  "schema_version": 0,
  "project_id": "proj-unknown-effect",
  "color_settings": {
    "working_space": "linear-srgb",
    "alpha_mode": "premultiplied"
  },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": {
        "1": "media/cel_0001.png",
        "2": "media/cel_0002.png"
      },
      "interpretation": {
        "color_space": "srgb",
        "alpha": "straight"
      }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 1920,
      "height": 1080,
      "pixel_aspect_ratio": 1,
      "frame_rate": {
        "numerator": 24,
        "denominator": 1
      },
      "start_frame": 0,
      "duration_frames": 5,
      "work_area": {
        "start_frame": 0,
        "end_frame_exclusive": 5
      },
      "layer_order": [
        "layer-cel"
      ],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 5,
          "source_offset_frames": 0,
          "transform": {
            "anchor": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "position": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "scale": {
              "base": [
                100,
                100
              ],
              "keyframes": []
            },
            "rotation": {
              "base": 0,
              "keyframes": []
            },
            "opacity": {
              "base": 1,
              "keyframes": []
            }
          },
          "exposure_spans": [
            {
              "start_frame": 0,
              "end_frame_exclusive": 2,
              "drawing_number": 1
            },
            {
              "start_frame": 2,
              "end_frame_exclusive": 5,
              "drawing_number": 2
            }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": [
            {
              "instance_id": "fx-unknown-1",
              "type_id": "vendor.future.effect",
              "enabled": true,
              "parameters": {
                "strength": 0.5
              },
              "opaque_unknown_data": {
                "keep_me": true
              }
            }
          ]
        }
      ]
    }
  ]
}
 | {
  "schema_version": 0,
  "project_id": "proj-unknown-effect",
  "color_settings": {
    "working_space": "linear-srgb",
    "alpha_mode": "premultiplied"
  },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": {
        "1": "media/cel_0001.png",
        "2": "media/cel_0002.png"
      },
      "interpretation": {
        "color_space": "srgb",
        "alpha": "straight"
      }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 1920,
      "height": 1080,
      "pixel_aspect_ratio": 1,
      "frame_rate": {
        "numerator": 24,
        "denominator": 1
      },
      "start_frame": 0,
      "duration_frames": 5,
      "work_area": {
        "start_frame": 0,
        "end_frame_exclusive": 5
      },
      "layer_order": [
        "layer-cel"
      ],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 5,
          "source_offset_frames": 0,
          "transform": {
            "anchor": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "position": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "scale": {
              "base": [
                100,
                100
              ],
              "keyframes": []
            },
            "rotation": {
              "base": 0,
              "keyframes": []
            },
            "opacity": {
              "base": 1,
              "keyframes": []
            }
          },
          "exposure_spans": [
            {
              "start_frame": 0,
              "end_frame_exclusive": 2,
              "drawing_number": 1
            },
            {
              "start_frame": 2,
              "end_frame_exclusive": 5,
              "drawing_number": 2
            }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": [
            {
              "instance_id": "fx-unknown-1",
              "type_id": "vendor.future.effect",
              "enabled": true,
              "parameters": {
                "strength": 0.5
              },
              "opaque_unknown_data": {
                "keep_me": true
              }
            }
          ]
        }
      ]
    }
  ]
}
 | pass |
| a project with no file of its own is not saved anywhere; the window asks | This project has no file of its own yet. Use Save As. | This project has no file of its own yet. Use Save As. | pass |
| a save that cannot be done says so in the core's words | true | true | pass |
| and does not leave a half-written file behind | false | false | pass |
| and the file that was already good is untouched | {
  "schema_version": 0,
  "project_id": "proj-unknown-effect",
  "color_settings": {
    "working_space": "linear-srgb",
    "alpha_mode": "premultiplied"
  },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": {
        "1": "media/cel_0001.png",
        "2": "media/cel_0002.png"
      },
      "interpretation": {
        "color_space": "srgb",
        "alpha": "straight"
      }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 1920,
      "height": 1080,
      "pixel_aspect_ratio": 1,
      "frame_rate": {
        "numerator": 24,
        "denominator": 1
      },
      "start_frame": 0,
      "duration_frames": 5,
      "work_area": {
        "start_frame": 0,
        "end_frame_exclusive": 5
      },
      "layer_order": [
        "layer-cel"
      ],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 5,
          "source_offset_frames": 0,
          "transform": {
            "anchor": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "position": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "scale": {
              "base": [
                100,
                100
              ],
              "keyframes": []
            },
            "rotation": {
              "base": 0,
              "keyframes": []
            },
            "opacity": {
              "base": 1,
              "keyframes": []
            }
          },
          "exposure_spans": [
            {
              "start_frame": 0,
              "end_frame_exclusive": 2,
              "drawing_number": 1
            },
            {
              "start_frame": 2,
              "end_frame_exclusive": 5,
              "drawing_number": 2
            }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": [
            {
              "instance_id": "fx-unknown-1",
              "type_id": "vendor.future.effect",
              "enabled": true,
              "parameters": {
                "strength": 0.5
              },
              "opaque_unknown_data": {
                "keep_me": true
              }
            }
          ]
        }
      ]
    }
  ]
}
 | {
  "schema_version": 0,
  "project_id": "proj-unknown-effect",
  "color_settings": {
    "working_space": "linear-srgb",
    "alpha_mode": "premultiplied"
  },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": {
        "1": "media/cel_0001.png",
        "2": "media/cel_0002.png"
      },
      "interpretation": {
        "color_space": "srgb",
        "alpha": "straight"
      }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 1920,
      "height": 1080,
      "pixel_aspect_ratio": 1,
      "frame_rate": {
        "numerator": 24,
        "denominator": 1
      },
      "start_frame": 0,
      "duration_frames": 5,
      "work_area": {
        "start_frame": 0,
        "end_frame_exclusive": 5
      },
      "layer_order": [
        "layer-cel"
      ],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 5,
          "source_offset_frames": 0,
          "transform": {
            "anchor": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "position": {
              "base": [
                0,
                0
              ],
              "keyframes": []
            },
            "scale": {
              "base": [
                100,
                100
              ],
              "keyframes": []
            },
            "rotation": {
              "base": 0,
              "keyframes": []
            },
            "opacity": {
              "base": 1,
              "keyframes": []
            }
          },
          "exposure_spans": [
            {
              "start_frame": 0,
              "end_frame_exclusive": 2,
              "drawing_number": 1
            },
            {
              "start_frame": 2,
              "end_frame_exclusive": 5,
              "drawing_number": 2
            }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": [
            {
              "instance_id": "fx-unknown-1",
              "type_id": "vendor.future.effect",
              "enabled": true,
              "parameters": {
                "strength": 0.5
              },
              "opaque_unknown_data": {
                "keep_me": true
              }
            }
          ]
        }
      ]
    }
  ]
}
 | pass |
| and the window is still showing the project it had | <a temporary directory>\saved_elsewhere.json | <a temporary directory>\saved_elsewhere.json | pass |

**15 of 15 checks pass.**

## What this does not cover

The dialogs. A file dialog belongs to the operating system and a test has no hands to answer one, so what is checked here begins at the path the person chose. Choosing a file, and the Open and Save As dialogs that do the choosing, are still unphotographed; the two photographs beside this table show a Ctrl+S save of a project that already had a file, which is the one path a script can drive from end to end.

Where a row says *a temporary directory*, the real value was this machine's scratch directory, which is different on every machine and on every run. The destination is shown rather than hidden — a save that reports the wrong one is exactly the failure worth seeing — but the machine-specific part of it is not, because this file is committed and checked.
