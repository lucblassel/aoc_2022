#!/usr/bin/env python3

import json

import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

def get_bench_set(path):
    with open(path, "r") as file:
        results = json.load(file)

    return [
        {"day": entry["command"].split("/")[-1], "time": time}
        for entry in results["results"]
        for time in entry["times"]
    ]

if __name__ == "__main__":

    results = []
    for path in ["./short-bench.json", "./medium-bench.json", "./long-bench.json"]:
        results.extend(get_bench_set(path))

    df = pd.DataFrame(results)

    order = sorted(df["day"].unique())

    plt.figure(figsize=(7,10))
    sns.stripplot(data=df, x="time", y="day", order=order)
    plt.xscale("log")
    plt.title("Benchmark times (in seconds)")
    plt.savefig("benchmarks.svg")