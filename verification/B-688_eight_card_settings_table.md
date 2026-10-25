# B-688 (D-808): 8 bpc (After Effects) on the card against the processor, W7's settings

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Shot: W7's, the reference shot at 1920 by 1080 with the effect on layer-2, set to 8 bpc; frames 30 and 120, Draft and Full
- Bar (D-808): at most 1 level of 255 on at least 99.9% of pixels, at most 2 anywhere

| Effect | Setting | On the card (Draft/Full) | Largest | Pixels over 1 | Pixels over 2 | Within 1 | Bar |
|---|---|---|---:|---:|---:|---:|---|
| (none) | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.exposure | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gaussian_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gaussian_blur | sigma_px 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.reduce_interlace_flicker | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.reduce_interlace_flicker | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.tint | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.line_smooth | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.selective_color_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.selective_color_blur | blur 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.glow | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.glow | radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.halation | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.halation | radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.line_recolor | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.directional_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.directional_blur | length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.select_color | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.line_width | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.radial_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bloom | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.bloom | radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_key | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_key | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.curves | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.levels | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.levels_individual | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.hue_saturation | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gradient | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.drop_shadow | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.drop_shadow | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.lens_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.lens_blur | radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.rim_light | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.rim_light | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.outline | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.outline | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.noise | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.chromatic_aberration | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.chromatic_aberration | fringe_blur 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.distance_gradation | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.light_rays | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.light_burst | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.exposure_flicker | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.vignette | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_vignette | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.turbulent_displace | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.turbulent_displace | complexity 4 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fractal_noise | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fractal_noise | complexity 8 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.turbulent_noise | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.turbulent_noise | complexity 12 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.gradient_map | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_balance | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.offset | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.light_wrap | as added | CPU/CPU | 0 | 0 | 0 | 100.0000% | met |
| core.invert | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.brightness_contrast | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.black_white | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.posterize | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.threshold | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.channel_mixer | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.vibrance | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.leave_color | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.leave_color | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.solarize | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.halftone | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.mosaic | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.emboss | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.find_edges | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.sharpen | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.sharpen | radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.diffusion | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.diffusion | radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.wave_warp | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.ripple | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.ripple | wavelength 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.twirl | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.twirl | radius 40, twirl_radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bulge | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bulge | vertical_radius 40, taper_radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.mirror | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.motion_tile | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.linear_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.linear_wipe | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.radial_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.radial_wipe | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.venetian_blinds | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.venetian_blinds | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.iris_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.iris_wipe | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.simple_choker | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.speed_lines | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cross_glare | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.camera_shake | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.rain | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.rain | length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_lookup | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.line_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.line_blur | length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.hsv_key | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.paraffin | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.kira_kira | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.lightning_bolt | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.compound_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.compound_blur | max_blur 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.displacement_map | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gradient_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gradient_wipe | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.echo | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cc_wide_time | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cc_wide_time | forward_steps 6, backward_steps 6 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.force_motion_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.force_motion_blur | motion_blur_samples 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.pixel_motion_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.pixel_motion_blur | shutter_samples 32, vector_detail 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.timewarp | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.timewarp | vector_detail 40, smoothing_iterations 8, shutter_samples 10 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.warp_stabilizer | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.rolling_shutter_repair | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.rolling_shutter_repair | pixel_motion_detail 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cc_rainfall | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cc_rainfall | spread 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.wave_world | as added | CPU/CPU | 0 | 0 | 0 | 100.0000% | met |
| core.caustics | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.caustics | bottom_blur 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.card_dance | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.shatter | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.time_difference | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.posterize_time | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.slider_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.checkbox_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.angle_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.point_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.layer_control | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.change_to_color | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.corner_pin | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.light_sweep | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.radio_waves | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.polar_coordinates | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.median | as added | CPU/card | 1 | 0 | 0 | 100.0000% | met |
| core.smart_blur | as added | CPU/card | 1 | 0 | 0 | 100.0000% | met |
| core.bilateral_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bilateral_blur | radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cross_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cross_blur | radius_x 40, radius_y 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.spin_zoom_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.spin_zoom_blur | quality 100 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fast_zoom_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.broadcast_safe | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_neutralizer | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_offset | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.kernel | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.toner | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.change_color | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.change_color | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_balance_hls | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_link | as added | CPU/CPU | 0 | 0 | 0 | 100.0000% | met |
| core.color_stabilizer | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gamma_pedestal_gain | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.photo_filter | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_grade | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.color_grade | key_hue_softness 40, key_softness 40, key_blur 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.snowfall | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.kaleidoscope | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.roughen_edges | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.roughen_edges | complexity 6 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.beam | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.beam | length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.four_color_gradient | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cell_pattern | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.optics_compensation | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.radial_shadow | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.radial_shadow | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.extract | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.extract | black_softness 40, white_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bevel_alpha | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bevel_edges | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.block_dissolve | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.block_dissolve | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.shift_channels | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.solid_composite | as added | CPU/CPU | 0 | 0 | 0 | 100.0000% | met |
| core.channel_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.channel_blur | red_blurriness 40, green_blurriness 40, blue_blurriness 40, alpha_blurriness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fast_box_blur | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.fast_box_blur | radius 40, iterations 6 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fast_blur_legacy | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.fast_blur_legacy | blurriness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.gaussian_blur_legacy | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.gaussian_blur_legacy | blurriness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.colorama | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.colorama | matching_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.glass | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.glass | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.vector_blur | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.vector_blur | map_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bend_it | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bender | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.blobbylize | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.blobbylize | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.glue_gun | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.threads | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.lens_flare | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.vegas | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.flow_motion | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.griddler | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fisheye | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.page_turn | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.page_turn | fold_radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.power_pin | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.bezier_warp | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.mesh_warp | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.warp | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.liquify | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.reshape | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.mask_smear | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.ripple_pulse | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.slant | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.smear | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.split | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.split_2 | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.tiles | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.magnify | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.magnify | feather 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.spherize | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.checkerboard | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.checkerboard | feather_width 40, feather_height 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.circle | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.circle | feather_outer 40, feather_inner 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.ellipse | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.detail_upscale | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.detail_upscale | detail 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.audio_spectrum | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.audio_waveform | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.audio_waveform | displayed_samples 64 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fractal | as added | card/card | 0 | 0 | 0 | 100.0000% | met |
| core.fractal | cycle_steps 20 | card/card | 0 | 0 | 0 | 100.0000% | met |
| core.grid | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.grid | feather_width 40, feather_height 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.fill | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.fill | horizontal_feather 40, vertical_feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.eyedropper_fill | as added | card/card | 0 | 0 | 0 | 100.0000% | met |
| core.eyedropper_fill | sample_radius 40 | card/card | 0 | 0 | 0 | 100.0000% | met |
| core.paint_bucket | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.paint_bucket | spread_radius 40, feather_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.write_on | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.write_on | stroke_length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.scribble | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.add_grain | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.noise_alpha | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.noise_hls | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.curl_noise | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.curl_noise | softness 40, flow_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.noise_hls_auto | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.dust_scratches | as added | CPU/card | 1 | 0 | 0 | 100.0000% | met |
| core.match_grain | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.remove_grain | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.remove_grain | passes 2, unsharp_radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.brush_strokes | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.brush_strokes | stroke_length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cartoon | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.deband | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.deband | edge_softness 40, blur_amount 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.advanced_spill_suppressor | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_difference_key | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.color_range | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.difference_matte | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.difference_matte | matching_softness 40, blur_before_difference 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.inner_outer_key | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.inner_outer_key | single_mask_highlight_radius 40, edge_feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.key_cleaner | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.key_cleaner | additional_edge_radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.linear_color_key | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.linear_color_key | matching_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.screen_key | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.screen_key | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.spill_suppressor | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.unmult | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.unmult | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.three_d_glasses | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.glass_melt | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.glass_melt | softness 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.tile_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.teeth_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.glow_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stripe_sweep | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.push_iris | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stretch_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.twister | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.warp_blend | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.channel_wipe | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.channel_wipe | softness 40, blur 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cc_spotlight | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_spotlight | edge_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_space | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_space | bounce_spread 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.bubbles | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.drizzle | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_plane | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_playground | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_playground | barrel_radius 40, direction_random_spread 40, velocity_random_spread 40, cannon_particle_radius 40, grid_particle_radius 40, explode_radius 40, particle_exploder_radius 40, gravity_random_spread 40, particle_exploder_age_feather 40, layer_map_age_feather 40, gravity_age_feather 40, wall_age_feather 40, persistent_age_feather 40, ephemeral_age_feather 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.hair | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.hair | length 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_emitter | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_emitter | direction_spread 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.particle_grid | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.mercury | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.mercury | radius_x 40, radius_y 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.polygon_shards | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.foam | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.scatterize | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.star_burst | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_ball_action | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.energy_stroke | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.energy_stroke | core_softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_cylinder | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_sphere | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.card_wipe | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.cc_environment | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.simple_wire_removal | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.block_load | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.burn_film | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.hex_tile | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.mr_smoothie | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.plastic | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.plastic | softness 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.repeat_tile | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.scatter | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.strobe_light | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.texturize | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.channel_threshold | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.threshold_rgb | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.contour_paint | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.contour_paint | smear_iterations 6 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.arbitrary_map | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.selective_color | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.shadow_highlight | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.shadow_highlight | shadow_radius 40, highlight_radius 40 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.tritone | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.aerial_haze | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.transform | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.moment_map | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.pass_extract | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.depth_key | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.depth_key | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.id_key | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.id_key | feather 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.cryptomatte | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.depth_of_field | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.depth_of_field | maximum_radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.fog_3d | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.identifier | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.numbers | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.basic_text | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.timecode | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.text_animator | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stretch_levels | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stretch_contrast | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stretch_color | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.spread_tones | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.matte_choker | as added | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card | 1 | 0 | 0 | 100.0000% | met |
| core.refine_hard_matte | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.refine_hard_matte | feather 40, decontamination_radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.refine_soft_matte | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.refine_soft_matte | edge_radius 40, feather 40, decontamination_radius 40 | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |
| core.stroke | as added | CPU/CPU | 1 | 0 | 0 | 100.0000% | met |

**369 of 369 rows meet the bar.**
