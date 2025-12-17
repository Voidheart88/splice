# Gekoppelte Induktivitäten in SPICE
# Syntax: K<name> L<name1> L<name2> <Kopplungsfaktor>
# Beispiel: K123 L1 L2 0.999
# Dabei ist der Kopplungsfaktor k = M / sqrt(L1*L2)
# M = k * sqrt(L1*L2)
# Die gegenseitige Induktivität M wird in die MNA-Matrix eingebaut

# Für die Implementierung benötigen wir:
# 1. Ein neues Element CoupledInductors oder MutualInductance
# 2. SPICE-Parser-Unterstützung für K-Elemente
# 3. Serde-Unterstützung
# 4. MNA-Integration für die gegenseitige Induktivität
# 5. Transiente Simulation mit Berücksichtigung der Kopplung

# Mathematische Grundlagen:
# v1 = L1 * di1/dt + M * di2/dt
# v2 = L2 * di2/dt + M * di1/dt
# In MNA-Form mit Backward Euler:
# i1 = (M/Δt) * (v1 - v2) + i1_prev + (M/Δt) * (v1_prev - v2_prev)
# i2 = (M/Δt) * (v2 - v1) + i2_prev + (M/Δt) * (v2_prev - v1_prev)

# Implementierungsplan:
# 1. Neues Modell in src/models/coupled_inductors/
# 2. SPICE-Parser-Erweiterung
# 3. Serde-Erweiterung
# 4. Integration in die Simulations-Engine
# 5. Tests
