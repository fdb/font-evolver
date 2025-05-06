#!/bin/sh
ffmpeg -y -loglevel warning -i results/best_%04d.png -pix_fmt yuv420p -r 30 results/best.mp4
