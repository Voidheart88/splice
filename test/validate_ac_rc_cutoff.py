#!/usr/bin/env python3
"""
Validate AC RC cutoff frequency test
RC Low-Pass Filter: R=1kΩ, C=1µF
Expected cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz
At fc, |Vout/Vin| should be ≈ 0.707 (3dB point)
"""

import subprocess
import sys
import math

def run_splice():
    """Run splice and capture output"""
    result = subprocess.run(
        ['./target/debug/splice', 'test/ac_rc_cutoff.cir'],
        capture_output=True,
        text=True,
        cwd='/home/dominik/git/splice'
    )
    
    if result.returncode != 0:
        print(f"Error running splice: {result.stderr}")
        sys.exit(1)
    
    return result.stdout

def parse_output(output):
    """Parse splice output"""
    lines = output.strip().split('\n')
    data = []
    
    for line in lines[1:]:  # Skip header
        parts = line.split(',')
        if len(parts) >= 5:
            freq = float(parts[0])
            v2_real = float(parts[2])
            v2_imag = float(parts[3])
            data.append((freq, v2_real, v2_imag))
    
    return data

def calculate_magnitude(v_real, v_imag):
    """Calculate magnitude of complex voltage"""
    return math.sqrt(v_real**2 + v_imag**2)

def find_cutoff_frequency(data):
    """Find frequency where |Vout/Vin| = 0.707"""
    target = 0.707
    
    for freq, v2_real, v2_imag in data:
        magnitude = calculate_magnitude(v2_real, v2_imag)
        if magnitude <= target:
            return freq, magnitude
    
    return None, None

def main():
    print("AC RC Cutoff Frequency Validation")
    print("=" * 50)
    print(f"RC Low-Pass Filter: R=1kΩ, C=1µF")
    print(f"Expected cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz")
    print(f"At fc, |Vout/Vin| should be ≈ 0.707 (3dB point)")
    print()
    
    # Run simulation
    output = run_splice()
    data = parse_output(output)
    
    # Find cutoff frequency
    cutoff_freq, magnitude = find_cutoff_frequency(data)
    
    if cutoff_freq:
        print(f"Measured cutoff frequency: {cutoff_freq:.2f} Hz")
        print(f"Magnitude at cutoff: {magnitude:.4f}")
        print(f"Expected magnitude: 0.7071")
        print()
        
        # Calculate error
        expected_fc = 159.15
        error_percent = abs(cutoff_freq - expected_fc) / expected_fc * 100
        
        print(f"Frequency error: {error_percent:.2f}%")
        
        if error_percent < 5:
            print("✅ TEST PASSED: Cutoff frequency within 5% of expected value")
            return 0
        else:
            print("❌ TEST FAILED: Cutoff frequency error too large")
            return 1
    else:
        print("❌ TEST FAILED: Could not find cutoff frequency")
        return 1

if __name__ == '__main__':
    sys.exit(main())