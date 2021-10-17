## About

Simple implementation of the [Haversine formula](https://en.wikipedia.org/wiki/Haversine_formula) for calculating
the distance between two locations on an [earth mean radius](https://en.wikipedia.org/wiki/Earth_radius) sphere.

## Usage

The program expects four arguments in a specific order, being:
* latitude of the first position
* longitude of the first position
* latitude of the second position
* longitude of the second position

where the values are given in [decimal degrees](https://en.wikipedia.org/wiki/Decimal_degrees).

The result is in *meters*.

### Example

The distance between the [Brisbane GPO](https://en.wikipedia.org/wiki/General_Post_Office,_Brisbane)
at 27.4679° S, 153.0280° E and the [Sydney GPO](https://en.wikipedia.org/wiki/General_Post_Office,_Sydney)
at 33.8676° S, 151.2075° E may be determined by running:
```bash
haversine -27.4679 152.028 -33.8676 151.2075
```

## Limitations

As this is intended to be a simple implementation no error checking is performed.
