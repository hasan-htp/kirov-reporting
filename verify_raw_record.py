import numpy as np
import sounddevice as sd
import os

SAMPLE_RATE = 16000


for e in os.scandir("raw_records/"):
    if e.is_file():
        with open(e.path, "rb") as f:
            data = f.read()
            samples = np.frombuffer(data, dtype=np.int16)

            print("Samples:", len(samples))
            print("Duration:", len(samples)/SAMPLE_RATE, "seconds")

            sd.play(samples, SAMPLE_RATE)
            sd.wait()
