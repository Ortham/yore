Yore
====

[![Travis Build Status](https://www.travis-ci.org/WrinklyNinja/yore.svg?branch=master)](https://www.travis-ci.org/WrinklyNinja/yore)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/WrinklyNinja/yore?branch=master&svg=true)](https://ci.appveyor.com/project/WrinklyNinja/yore)
[![dependency status](https://deps.rs/repo/github/WrinklyNinja/yore/status.svg)](https://deps.rs/repo/github/WrinklyNinja/yore)
[![Greenkeeper badge](https://badges.greenkeeper.io/WrinklyNinja/yore.svg)](https://greenkeeper.io/)

A cross-platform utility to help geotag JPEG photos using your Google
Location History data.

Given a Google Location History JSON file and a directory, Yore can
recursively scan the directory for images without GPS metadata and match their
timestamps to location timestamps, optionally interpolating between data points
or picking the point closest in time when an exact match doesn't exist.

Yore relies on [Exiv2](http://www.exiv2.org/) to write GPS metadata.

## Build

To build Yore, install [Rust](https://www.rust-lang.org) and
[Node 8+](http://nodejs.org/) then run

```
npm install
npm run build
cargo build --release
```

to create release executables at `target/release/yore` and
`target/release/yore-webview` (`yore.exe` and `yore-webview.exe` on Windows).

## Usage

Before using Yore, you need to
[export](https://takeout.google.com/settings/takeout/custom/location_history)
your Google Location History data as a JSON file.

Also ensure that you have [Exiv2](http://www.exiv2.org/) installed and that its
executables are on your PATH or in the same directory as your `yore` and/or
`yore-webview` executable(s). You don't need Exiv2 installed if you aren't going
to apply any suggested locations.

Suggestions are made by finding the closest match to the photo's date taken
timestamp in the location history data. The accuracy distance is as recorded by
Google, and may not itself be particularly accurate. The accuracy time is the
difference between the photo and location timestamps: negative values are when
the suggested location timestamp is older than the photo timestamp.

Suggestions are not made for photos without date taken timestamps or photos
which already have location metadata. In the latter case, the existing metadata
will be displayed instead.

### Webview

Just run `yore-webview`. Alternatively, you can set the initial values for the
photo and location history file paths and whether or not location suggestions
are interpolated, using command line parameters:

```
./yore-webview -i -l LocationHistory.json photos/
```

### CLI

To get a suggested location for a single photo:

```
./yore -l LocationHistory.json photo.jpg

"photo.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576%2C12.6572722>

Save the suggested location to this image? (y/n)
```

To recursively scan a directory for photos and get suggested locations for
them:

```
./yore -l LocationHistory.json photos/

"photos/folder1/photo1.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576%2C12.6572722>

Save the suggested location to this image? (y/n)
y
Location saved for folder1/photo1.jpg

"photos/folder2/photo2.jpg":
        Suggested location: (55.638164, 12.6563669)
        Suggestion accuracy: 21 metres, 1 minute, 59 seconds
        View on map: <https://www.google.co.uk/maps/place/55.638164%2C12.6563669>

Save the suggested location to this image? (y/n)
n

"photos/folder2/photo3.jpg":
        Already has a location: (38.76544, -9.094802222222222)
```

To get location suggestions and skip prompts to save them:

```
./yore -r -l LocationHistory.json photos/

"photos/folder1/photo1.jpg":
        Suggested location: (55.6382576, 12.6572722)
        Suggestion accuracy: 6 metres, -3 seconds
        View on map: <https://www.google.co.uk/maps/place/55.6382576%2C12.6572722>

"photos/folder2/photo2.jpg":
        Suggested location: (55.638164, 12.6563669)
        Suggestion accuracy: 21 metres, 1 minute, 59 seconds
        View on map: <https://www.google.co.uk/maps/place/55.638164%2C12.6563669>

"photos/folder2/photo3.jpg":
        Already has a location: (38.76544, -9.094802222222222)
```

### Browser-based GUI

The CLI can also be used to start a server for the GUI that can then be used via
a web browser.

```
./yore -g -l LocationHistory.json photos/
Listening on http://127.0.0.1:8080
```

Once Yore is listening, open your web browser to the URL Yore is
listening on, and you'll see Yore's GUI.

The port Yore listens on can be set using the `-p` CLI argument, and defaults to
`8080`. A value of `0` will bind the server to an ephemeral port. When running
the GUI server, the `-r` flag is ignored, and the location history and photo
path parameters are optional.

### Interpolation

To enable interpolation, pass the `-i` flag when running Yore.

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
