from matplotlib import pyplot as plt
import json
from rich import print
import os

for file_name in os.listdir("logs"):
    with open(f"logs/{file_name}", "r") as f:
        lengths = []
        times = []
        for line in f.readlines():
            line = line.replace("\n", "")
            line_data = []
            key_value_pairs = line.split(";")
            length = int(key_value_pairs[0].split("=")[1])
            time = float(key_value_pairs[1].split("=")[1])
            lengths.append(length)
            times.append(time)

    plt.plot(lengths, times, label=file_name.replace("_benchmark.log", "").replace("_", " ").title())

plt.legend(loc="upper left")
plt.show()

