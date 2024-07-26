'''Generates the Spice TestFile'''

def generate_spice_file(num_combinations, filename):
    '''Generates the File'''
    with open(filename, 'w',encoding="utf-8") as f:
        f.write("V1 0 N0 10\n")
        for i in range(1, num_combinations + 1):
            f.write(f"R{i} N{i-1} N{i} 10\n")
            f.write(f"D{i} N{i} 0 diode\n")
        f.write(".op")


NUM_COMBINATIONS = 20000
FILENAME = "rd-ladder-20k.cir"
generate_spice_file(NUM_COMBINATIONS, FILENAME)
print(f"SPICE file '{FILENAME}' generated with {NUM_COMBINATIONS} resistor-diode combinations.")
