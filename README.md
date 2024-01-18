# ItsyBitsy M4 Express LED Controller

## Overview
This Rust-based project is specifically designed for flashing the ItsyBitsy M4 Express microcontroller, enabling it to receive events over USB and execute associated animations on hundreds of connected LEDs. It's part of the larger project, MIKE (Mycorrhizal Interactive Kinetic Exhibit), developed under the Industrial Design program at TU Delft.

## Context of the Project
This work serves as a rewrite of the original CircuitPython code used in the MIKE project, substantially enhancing the rate of animation and increasing the total number of possible animations. For more details on the original project, visit [IE-minor](https://github.com/sashalikesplanes/IE-minor).

### MIKE - Mycorrhizal Interactive Kinetic Exhibit
MIKE is an interactive system designed to mimic mycorrhizal networks, using LED lights and infrared cameras for human interaction. Key features of MIKE include:
- C++ program for infrared camera data processing and object detection.
- TypeScript controller for behavior triggering based on human detection.
- Embedded program on a microcontroller, originally using CircuitPython for LED updates.

## Features
- **Rust for Reliability**: Leveraging Rust's safety and performance for robust firmware.
- **USB Event Handling**: Efficiently processes events received via USB.
- **Dynamic LED Control**: Manages hundreds of LEDs for complex animations.

## Requirements
- ItsyBitsy M4 Express Microcontroller
- Rust Environment Setup
- USB Connection to Microcontroller
- LED Strip (compatible with ItsyBitsy M4 Express)
