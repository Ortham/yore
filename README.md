Yore
====

[![Build Status](https://www.travis-ci.org/WrinklyNinja/yore.svg?branch=master)](https://www.travis-ci.org/WrinklyNinja/yore)
[![Coveralls branch](https://img.shields.io/coveralls/WrinklyNinja/yore/master.svg)](https://coveralls.io/github/WrinklyNinja/yore)

A cross-platform CLI utility to help geotag JPEG photos using your Google
Location History data.

Given a Google Location History JSON file and a directory, Yore can
recursively scan the directory for images without GPS metadata and match their
timestamps to location timestamps, optionally interpolating between data points
or picking the point closest in time when an exact match doesn't exist.

Yore relies on [Exiv2](http://www.exiv2.org/) to write GPS metadata.

## Usage

Before using Yore, you need to
[export](https://takeout.google.com/settings/takeout/custom/location_history)
your Google Location History data as a JSON file.

Also ensure that you have [Exiv2](http://www.exiv2.org/) installed and that its
executables are on your PATH or in the same directory as your `yore` executable.
You don't need Exiv2 installed if you don't attempt to save any suggested
locations.

Then, to get a suggested location for a single photo:

```
./yore -l LocationHistory.json photo.jpg

"photo.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576,12.6572722>

Save the suggested location to this image? (y/n)
```

Or to recursively scan a directory for photos and get suggested locations for
them:

```
./yore -l LocationHistory.json photos/

"photos/folder1/photo1.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576,12.6572722>

Save the suggested location to this image? (y/n)
y
Location saved for folder1/photo1.jpg

"photos/folder2/photo2.jpg":
        Suggested location: (55.638164, 12.6563669)
        Suggestion accuracy: 21 metres, 1 minute, 59 seconds
        View on map: <https://www.google.co.uk/maps/place/55.638164,12.6563669>

Save the suggested location to this image? (y/n)
n

"photos/folder2/photo3.jpg":
        Already has a location: (38.76544, -9.094802222222222)
```

Suggestions are made by finding the closest match to the photo's date taken
timestamp in the location history data. The accuracy distance is as recorded by
Google, and may not itself be particularly accurate. The accuracy time is the
difference between the photo and location timestamps: negative values are when
the suggested location timestamp is older than the photo timestamp.

Suggestions are not made for photos without date taken timestamps or photos
which already have location metadata. In the latter case, the existing metadata
will be displayed instead.

### Interpolation

If interpolation is enabled and a photo's timestamp doesn't exactly match a
location timestamp but is at a time between two location data points, the
location will be calculated by assuming movement in a straight line at constant
radial speed between the two points. The use of radial speed may make results
less accurate when the two locations are at significantly different latitudes,
but in such cases the result will be pretty inaccurate anyway.

With interpolation enabled the suggestion accuracy is calculated by linearly
interpolating between the recorded accuracies of the two location data points
and half the surface distance between the two locations. The accuracy starts
at the value of the preceding location's accuracy, scales to the half-distance
value at the mid-point between the two locations, and scales to the value of the
following location's accuracy.

If the half-distance is less than either location's accuracy, it is ignored and
the accuracy is linearly interpolated between the two location accuracies.
