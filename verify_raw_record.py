import numpy as np
import sounddevice as sd
import os
import sys

SAMPLE_RATE = 48000
CHANNELS = 2

if len(sys.argv) < 2:
    print(f"Usage: {sys.argv[0]} <raw_records_dir>")
    sys.exit(1)

raw_dir = sys.argv[1]

for e in sorted(os.scandir(raw_dir), key=lambda x: x.name):
    if e.is_file() and e.name.endswith(".raw"):
        print(f"\nPlaying: {e.name}")
        with open(e.path, "rb") as f:
            data = f.read()
        samples = np.frombuffer(data, dtype=np.int16)
        samples = samples.reshape(-1, CHANNELS)
        sd.play(samples, SAMPLE_RATE)
        sd.wait()