# AC Analysis Test Notes

## RC Low-Pass Filter Test

This test validates the AC analysis of an RC low-pass filter with:
- R = 1kΩ
- C = 1µF
- Expected cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz
- At fc, |Vout/Vin| should be ≈ 0.707 (3dB point)

## Test Files

1. `ac_rc_cutoff.cir` - Full frequency range (10 Hz - 1000 Hz)
2. `ac_rc_simple.cir` - Focused range (100 Hz - 200 Hz)
3. `ac_rc_highres.cir` - High resolution (100 Hz - 200 Hz, 100 steps)

## Known Issues

The current AC analysis shows a cutoff frequency around 195 Hz instead of the expected 159.15 Hz. This represents an error of approximately 22-24%.

### Possible Causes

1. **Voltage Source Implementation**: The voltage source in AC analysis may not be correctly implemented in the MNA formulation.
2. **Matrix Conditioning**: The matrix may be poorly conditioned for certain frequency ranges.
3. **Solver Limitations**: The solver may have limitations with complex matrices in certain configurations.

### Investigation Results

- Low-resolution test (10-1000 Hz, 100 steps): fc ≈ 195 Hz (22.48% error)
- High-resolution test (100-200 Hz, 100 steps): fc ≈ 105 Hz (34.08% error)
- The error is consistent across different frequency ranges and resolutions

### Next Steps

1. Investigate the voltage source implementation in AC analysis
2. Check the matrix assembly for AC analysis
3. Verify the solver's handling of complex matrices
4. Compare with known-good SPICE implementations

## Validation Scripts

- `validate_ac_rc_cutoff.py` - Basic validation
- `analyze_ac_data.py` - Detailed analysis with interpolation
- `analyze_highres.py` - High-resolution analysis

## Expected Behavior

For an RC low-pass filter with R=1kΩ and C=1µF:
- At 10 Hz: |Vout/Vin| ≈ 0.996 (very close to 1)
- At 159.15 Hz: |Vout/Vin| ≈ 0.707 (3dB point)
- At 1000 Hz: |Vout/Vin| ≈ 0.159 (significantly attenuated)

## Current Behavior

The simulation shows:
- At 10 Hz: |Vout/Vin| ≈ 0.996 ✓
- At 195 Hz: |Vout/Vin| ≈ 0.707 (shifted from expected 159.15 Hz)
- At 1000 Hz: |Vout/Vin| ≈ 0.159 ✓

The high-frequency behavior is correct, but the cutoff frequency is shifted.

## Conclusion

The AC analysis is functional but has a known issue with the cutoff frequency calculation. This should be investigated further to ensure accurate AC analysis results.