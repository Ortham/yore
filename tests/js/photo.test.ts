import * as renderer from 'react-test-renderer';
import { Coordinates, LocationAccuracy } from '../../src/gui/js/interfaces';

jest.mock('react-icons/fi', () => ({
  FiAlertCircle: 'FiAlertCircle',
  FiNavigation: 'FiNavigation',
  FiMapPin: 'FiMapPin'
}));

/* eslint-disable import/first */
import {
  locationDescription,
  hasSuggestion,
  googleMapsCoordinates,
  chooseIcon,
  updatePhotoLocations
} from '../../src/gui/js/photo';
/* eslint-enable import/first */

describe('locationDescription()', () => {
  test('returns suggested location if photo has one', () => {
    const photo = {
      path: '',
      location: {
        Suggested: [
          {},
          {
            meters: 5,
            seconds: 20
          }
        ] as [Coordinates, LocationAccuracy]
      }
    };
    expect(locationDescription(photo)).toBe(
      'Suggested location: accuracy is 5 meters and 20 seconds'
    );
  });

  test('returns existing location if photo has a location but no suggestion', () => {
    const photo = {
      path: '',
      location: {}
    };
    expect(locationDescription(photo)).toBe('Existing location');
  });

  test('returns error if photo has one', () => {
    const photo = {
      path: '',
      error: 'Oh no!'
    };
    expect(locationDescription(photo)).toBe(photo.error);
  });

  test('returns no location if photo has no location or error', () => {
    const photo = {
      path: ''
    };
    expect(locationDescription(photo)).toBe('No location');
  });
});

describe('hasSuggestion()', () => {
  test('returns false if photo has no location', () => {
    const photo = {
      path: ''
    };
    expect(hasSuggestion(photo)).toBe(false);
  });

  test('returns false if photo has a location but no Suggested key', () => {
    const photo = {
      path: '',
      location: {}
    };
    expect(hasSuggestion(photo)).toBe(false);
  });

  test('returns true if photo has a suggested location', () => {
    const photo = {
      path: '',
      location: {
        Suggested: [{}, {}] as [Coordinates, LocationAccuracy]
      }
    };
    expect(hasSuggestion(photo)).toBe(true);
  });
});

describe('googleMapsCoordinates()', () => {
  test("returns the photo's existing coordinates if it has them", () => {
    const photo = {
      path: '',
      location: {
        Existing: {
          latitude: 52.0,
          longitude: 13.2
        }
      }
    };
    const coordinates = googleMapsCoordinates(photo);
    expect(coordinates.lat).toBe(52.0);
    expect(coordinates.lng).toBe(13.2);
  });

  test("returns the photo's suggested coordinates if it has them", () => {
    const photo = {
      path: '',
      location: {
        Suggested: [
          {
            latitude: 52.0,
            longitude: 13.2
          },
          {
            meters: 5,
            seconds: 20
          }
        ] as [Coordinates, LocationAccuracy]
      }
    };
    const coordinates = googleMapsCoordinates(photo);
    expect(coordinates.lat).toBe(52.0);
    expect(coordinates.lng).toBe(13.2);
  });

  test('returns (0,0) if the photo has no location', () => {
    const photo = {
      path: ''
    };
    const coordinates = googleMapsCoordinates(photo);
    expect(coordinates.lat).toBe(0);
    expect(coordinates.lng).toBe(0);
  });
});

describe('chooseIcon()', () => {
  test('returns an exclamation circle element if the photo has an error', () => {
    const photo = {
      path: '',
      error: 'Oh no!'
    };
    const icon = renderer.create(chooseIcon(photo)).toJSON();
    expect(icon).toMatchSnapshot();
  });

  test('returns a map marker element if the photo has an existing location', () => {
    const photo = {
      path: '',
      location: {
        Existing: {} as Coordinates
      }
    };
    const icon = renderer.create(chooseIcon(photo)).toJSON();
    expect(icon).toMatchSnapshot();
  });

  test('returns a location arrow element if the photo has a suggested location', () => {
    const photo = {
      path: '',
      location: {
        Suggested: [{}, {}] as [Coordinates, LocationAccuracy]
      }
    };
    const icon = renderer.create(chooseIcon(photo)).toJSON();
    expect(icon).toMatchSnapshot();
  });

  test('returns null if the photo has no error or location', () => {
    const photo = {
      path: ''
    };
    expect(chooseIcon(photo)).toBe(null);
  });

  describe('updatePhotoLocations()', () => {
    test('throws if the lengths of photos and locations arrays are different', () => {
      const photos = [{ path: 'A' }, { path: 'B' }];
      const locations = [{}];

      expect(() => {
        updatePhotoLocations(photos, locations);
      }).toThrow('photos and locations array lengths are not equal');
    });

    test('creates a new array of new photo objects', () => {
      const photos = [{ path: 'A' }, { path: 'B' }];
      const locations = [{}, {}];

      const updatedPhotos = updatePhotoLocations(photos, locations);

      expect(updatedPhotos.length).toBe(photos.length);
      expect(updatedPhotos).not.toBe(photos);
      expect(updatedPhotos[0]).not.toBe(photos[0]);
      expect(updatedPhotos[1]).not.toBe(photos[1]);
    });

    test('merges the location object into each photo object', () => {
      const photos = [{ path: 'A' }, { path: 'B' }];
      const locations = [
        {
          location: {
            Existing: {
              latitude: 1,
              longitude: 2
            }
          }
        },
        { error: 'bar' }
      ];

      const updatedPhotos = updatePhotoLocations(photos, locations);

      expect(updatedPhotos[0].location).toBe(locations[0].location);
      expect(updatedPhotos[0].error).toBe(undefined);
      expect(updatedPhotos[1].location).toBe(undefined);
      expect(updatedPhotos[1].error).toBe(locations[1].error);
    });

    test('sets loaded to true for each photo', () => {
      const photos = [{ path: 'A' }, { path: 'B' }];
      const locations = [{}, {}];

      const updatedPhotos = updatePhotoLocations(photos, locations);

      expect(updatedPhotos[0].loaded).toBe(true);
      expect(updatedPhotos[1].loaded).toBe(true);
    });
  });
});
