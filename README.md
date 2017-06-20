Yore
====

[![Build Status](https://www.travis-ci.org/WrinklyNinja/yore.svg?branch=master)](https://www.travis-ci.org/WrinklyNinja/yore)
[![Coveralls branch](https://img.shields.io/coveralls/WrinklyNinja/yore/master.svg)](https://coveralls.io/github/WrinklyNinja/yore)

A utility for geotagging JPEG photos using your Google Location History data.

Yore doesn't currently support writing EXIF tags, so locations will only be
suggested.

## Usage

Before using Yore, you need to
[export](https://takeout.google.com/settings/takeout/custom/location_history)
your Google Location History data as a JSON file.

Then, to get a suggested location for a single photo:

```
./yore -l LocationHistory.json photo.jpg
"photo.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576,12.6572722>
```

Or to recursively scan a directory for photos and get suggested locations for
them:

```
./yore -l LocationHistory.json photos/
"folder1/photo1.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576,12.6572722>
"folder2/photo2.jpg":
        Suggested location: (55.638164, 12.6563669)
        Suggestion accuracy: 21 metres, 1 minute, 59 seconds
        View on map: <https://www.google.co.uk/maps/place/55.638164,12.6563669>
```

Suggestions are made by finding the closest match to the photo's date taken
timestamp in the location history data. The accuracy distance is as recorded by
Google, and may not itself be particularly accurate. The accuracy time is the
difference between the photo and location timestamps: negative values are when
the suggested location timestamp is older than the photo timestamp.

Suggestions are not made for photos without date taken timestamps or photos
which already have location metadata. In the latter case, the existing metadata
will be displayed instead.
