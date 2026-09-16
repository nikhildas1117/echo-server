import matplotlib.pyplot as plt

with open("rtt.csv", "r") as file:
    rtts = [float(line.strip()) for line in file if line.strip()]

plt.hist(rtts, bins=100)

plt.xlabel("Round-Trip Latency (ms)")
plt.ylabel("Frequency")
plt.title("Distribution of TCP Round-Trip Latency")

plt.show()