# RNG visualizer

Visualizer of random numbers generated by a simple random number generator

## Use

Asks for 3 inputs (leave empty to set to default):
- Shift: determines the lenght of the seed
- Seed: the initial internal state of the generator
- Range: determines how many numbers should be generated

And based on these inputs prints series of 1 bit numbers represented as dark or white spot (starting from the least significant bit of the seed).
Then waits until "exit" or "quit" is inputed and closes itself.

## Algorithm

It is based on variation of [Xorshift](https://en.wikipedia.org/wiki/Xorshift) algorithm.

This is the code responsible for generating a new seed and the output (displayed number):

    seed >>= 1;
    current_output = seed & 1;
    seed |= (previous_output ^ current_output) << shift;
    previous_output = current_output;
