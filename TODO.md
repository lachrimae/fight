- Host local inputs in a circular buffer for 3 frames
- Send the next 3 frames' worth of inputs to the other computer
- Synchronize clocks with the other computer
- Create a virtual event loop so that it can be fast-forwarded multiple times in a single FixedUpdate loop.
- The engine-level FixedUpdate spaces out the user's inputs, but in fact we rely on the Update loop to be able to speed up simulation.
