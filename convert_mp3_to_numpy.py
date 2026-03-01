import sys
import os
from audio2numpy import open_audio
import numpy as np

if len(sys.argv) != 3:
    print("Usage: python convert_mp3_to_numpy.py <input_path> <output_path>")
    sys.exit(1)

input_path = sys.argv[1]
output_path = sys.argv[2]

os.makedirs(output_path, exist_ok=True)

for filename in os.listdir(input_path):
    if filename.endswith(".mp3") or filename.endswith(".wav"):
        file_path = os.path.join(input_path, filename)
        signal, sampling_rate = open_audio(file_path)

        base_name = os.path.splitext(filename)[0]
        out_file_path = os.path.join(output_path, f"{base_name}.txt")

        np.savetxt(out_file_path, signal, fmt='%4.10f')
        print(f"Saved {out_file_path}, sampling rate: {sampling_rate}")