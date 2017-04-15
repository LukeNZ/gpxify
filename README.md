# gpxify

Geotag photos from GPSless cameras with linearly-interpolated GPX tracklog data from your handheld GPS.

## How to use

Basic usage:

```
gpxify
```

from a directory containing both a GPX track log (or logs) and a photo (or photos).

## Options

```
-t, --tracks "" \\ Optional, specify a custom track log (or directory of tracklogs).
-p, --photos "" \\ Optional, directory of photos.
-n, --note ""   \\ Optional, append a note to the UserComment EXIF field.
```

Usage:

```
gpxify [[-t|--tracks] option_argument][[-p|--photos] option_argument][[[-n|--note] option_argument]
```

### Future options

```
-r, --recurse
-o, --overwrite
```

# Accuracy

The accuracy of gpxify is determined by:

* The quality and accuracy of the GPS model you have recording.
* How frequently your GPS is appending new positional data to your running tracklog.
* How close you keep your GPS to your camera.

In most situations, photos geotagged by GPXify will be at least as accurate, if not moreso, than the GPS position recorded by a camera, provided a reasonable attempt is to made to optimize the three variables above.

Photos where there is no GPX data available cannot be tagged.

# How it works
