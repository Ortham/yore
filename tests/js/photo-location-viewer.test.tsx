import * as React from 'react';
import * as renderer from 'react-test-renderer';
import { Photo, LocationAccuracy } from '../../src/gui/js/interfaces';

jest.mock('../../src/gui/js/map-area', () => ({
  MapArea: 'MapArea'
}));

import { PhotoLocationViewer } from '../../src/gui/js/photo-location-viewer'; // eslint-disable-line import/first

function apply() {
  return Promise.resolve();
}

function discard() {}

describe('PhotoLocationViewer', () => {
  test('renders an image, map, location description and disabled buttons if photo location is not suggested', () => {
    const photo: Photo = {
      path: 'path',
      height: 400,
      width: 500,
      location: {
        Existing: {
          latitude: 52.0,
          longitude: 36.2
        }
      }
    };
    const photoLocationViewer = renderer
      .create(
        <PhotoLocationViewer
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(photoLocationViewer).toMatchSnapshot();
  });

  test('renders an image, map, text and disabled buttons if photo has no location', () => {
    const photo = {
      path: 'path',
      height: 400,
      width: 500
    };
    const photoLocationViewer = renderer
      .create(
        <PhotoLocationViewer
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(photoLocationViewer).toMatchSnapshot();
  });

  test('renders an image, map, location description and enabled buttons if photo location is suggested', () => {
    const photo = {
      path: 'path',
      height: 400,
      width: 500,
      location: {
        Suggested: [
          {
            latitude: 52.0,
            longitude: 36.2
          },
          {
            meters: 5,
            seconds: 10
          }
        ] as [Coordinates, LocationAccuracy]
      }
    };
    const photoLocationViewer = renderer
      .create(
        <PhotoLocationViewer
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(photoLocationViewer).toMatchSnapshot();
  });

  test('renders text and disabled buttons if photo is undefined', () => {
    const photoLocationViewer = renderer
      .create(
        <PhotoLocationViewer
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(photoLocationViewer).toMatchSnapshot();
  });
});
