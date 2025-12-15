#!/usr/bin/env python3

import numpy as np
import matplotlib.pyplot as plt

# Simple RC circuit simulation for comparison
# V1 = 5V, R1 = 1000Ω, C1 = 0.001F
# Time constant τ = R*C = 1s

R = 1000
C = 0.001
tau = R * C
V_source = 5.0

# Simulation parameters
dt = 0.0001  # 0.1ms time step
t_end = 0.01  # 10ms simulation time
time = np.arange(0, t_end + dt, dt)

# Analytical solution: V_c(t) = V_source * (1 - exp(-t/tau))
V_analytical = V_source * (1 - np.exp(-time / tau))

# Numerical solution using backward Euler
V_num = np.zeros_like(time)
V_num[0] = 0.0  # Initial voltage

for i in range(1, len(time)):
    # Backward Euler: V_c[n] = (V_source/R + C*V_c[n-1]/dt) / (1/R + C/dt)
    numerator = V_source / R + C * V_num[i-1] / dt
    denominator = 1/R + C / dt
    V_num[i] = numerator / denominator

# Plot results
plt.figure(figsize=(12, 6))
plt.plot(time, V_analytical, 'b-', label='Analytical')
plt.plot(time, V_num, 'r--', label='Numerical (Backward Euler)')
plt.xlabel('Time (s)')
plt.ylabel('Capacitor Voltage (V)')
plt.title('RC Circuit Charging (R=1kΩ, C=1mF, τ=1s)')
plt.legend()
plt.grid(True)
plt.show()

print("Expected capacitor voltage at t=0.01s:", V_analytical[-1], "V")
print("Numerical capacitor voltage at t=0.01s:", V_num[-1], "V")