#!/usr/bin/env python3
"""
Analyze high-resolution AC data
"""

import math
import csv

def read_data(filename):
    """Read AC data from CSV file"""
    data = []
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        next(reader)  # Skip header
        for row in reader:
            freq = float(row[0])
            v2_real = float(row[2])
            v2_imag = float(row[3])
            data.append((freq, v2_real, v2_imag))
    return data

def calculate_magnitude(v_real, v_imag):
    """Calculate magnitude of complex voltage"""
    return math.sqrt(v_real**2 + v_imag**2)

def find_cutoff_frequency(data):
    """Find frequency where |Vout/Vin| = 0.707 using linear interpolation"""
    target = 0.707
    
    prev_freq, prev_mag = None, None
    for freq, v2_real, v2_imag in data:
        magnitude = calculate_magnitude(v2_real, v2_imag)
        
        if prev_mag is not None and magnitude < target and prev_mag > target:
            # Linear interpolation between previous and current point
            ratio = (target - prev_mag) / (magnitude - prev_mag)
            interpolated_freq = prev_freq + ratio * (freq - prev_freq)
            return interpolated_freq, magnitude
        
        prev_freq, prev_mag = freq, magnitude
    
    return None, None

def main():
    print("High-Resolution AC RC Cutoff Frequency Analysis")
    print("=" * 60)
    print(f"RC Low-Pass Filter: R=1kΩ, C=1µF")
    print(f"Expected cutoff frequency: fc = 1/(2πRC) ≈ 159.15 Hz")
    print(f"At fc, |Vout/Vin| should be ≈ 0.707 (3dB point)")
    print()
    
    # Read data
    data = read_data('/tmp/ac_highres.csv')
    
    # Find cutoff frequency with interpolation
    cutoff_freq, magnitude = find_cutoff_frequency(data)
    
    if cutoff_freq:
        print(f"Measured cutoff frequency (interpolated): {cutoff_freq:.2f} Hz")
        print(f"Magnitude at cutoff: {magnitude:.4f}")
        print(f"Expected magnitude: 0.7071")
        print()
        
        # Calculate error
        expected_fc = 159.15
        error_percent = abs(cutoff_freq - expected_fc) / expected_fc * 100
        
        print(f"Frequency error: {error_percent:.2f}%")
        
        if error_percent < 10:
            print("✅ TEST PASSED: Cutoff frequency within 10% of expected value")
            return 0
        else:
            print("❌ TEST FAILED: Cutoff frequency error too large")
            return 1
    else:
        print("❌ TEST FAILED: Could not find cutoff frequency")
        return 1

if __name__ == '__main__':
    import sys
    sys.exit(main())