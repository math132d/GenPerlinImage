## GenPerlinImage
__A simple rust program to generate images from perlin noise.__ This project was originally a part of [another](https://github.com/math132d/rust_perlin) project. But were split up to better utilize [rust_perlin](https://github.com/math132d/rust_perlin) as just a library.

### Example of use
You can run the program using the following command
```sh
$ ./perlin_image 256 256 image.png -f 6 -o 4
```
* `256 256 image.png` Are __required__ and determines the size and filename of the image
* `-f 6 -o 4` Are __optional__, and sets the Frequency _(Rate of change over the image)_ and Octaves _(Detail)_ respectively

### Getting the program

#### Download

There are downloads availible for:

* [Windows](../../raw/master/release/perlin_image.exe)__(coming soon)__
* [Linux](../../raw/master/release/perlin_image)

#### Build yourself

To get the program clone the repository to a directory of your choice.
```sh
$ git clone https://github.com/math132d/GenPerlinImage.git
$ cd ./GenPerlinNoise/
```

And compile the program.

_(For this step you should have rust and cargo installed. see [getting started](https://www.rust-lang.org/learn/get-started))_
```sh
$ cargo build --release
```

The build should be availible under `GenPerlinNoise/target/release/image_perlin` or `image_perlin.exe` depending on your system.

