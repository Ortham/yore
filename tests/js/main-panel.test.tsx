import * as React from 'react';
import * as renderer from 'react-test-renderer';
import { LocationAccuracy } from '../../src/gui/js/interfaces';

jest.mock('../../src/gui/js/map-area', () => {
  return {
    'MapArea': 'MapArea'
  }
});

import { MainPanel } from '../../src/gui/js/main-panel'; // eslint-disable-line import/first

function apply() {
  return Promise.resolve();
}

function discard() {}

describe('MainPanel', () => {
  test('renders an image, map, location description and disabled buttons if photo location is not suggested', () => {
    const photo = {
      src: 'path',
      path: '',
      location: {
        Existing: {
          latitude: 52.0,
          longitude: 36.2
        }
      }
    };
    const mainPanel = renderer
      .create(
        <MainPanel
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(mainPanel).toMatchSnapshot();
  });

  test('renders an image, map, text and disabled buttons if photo has no location', () => {
    const photo = {
      src: 'path',
      path: ''
    };
    const mainPanel = renderer
      .create(
        <MainPanel
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(mainPanel).toMatchSnapshot();
  });

  test('renders an image, map, location description and enabled buttons if photo location is suggested', () => {
    const photo = {
      src: 'path',
      path: '',
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
    const mainPanel = renderer
      .create(
        <MainPanel
          photo={photo}
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(mainPanel).toMatchSnapshot();
  });

  test('renders text and disabled buttons if photo is undefined', () => {
    const mainPanel = renderer
      .create(
        <MainPanel
          handleSuggestionApply={apply}
          handleSuggestionDiscard={discard}
        />
      )
      .toJSON();
    expect(mainPanel).toMatchSnapshot();
  });
});
