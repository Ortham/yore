import * as React from 'react';
import * as renderer from 'react-test-renderer';
import { Photo } from '../../src/gui/js/interfaces';

jest.mock('google-map-react', () => ({
  default: 'GoogleMapReact'
}));
jest.mock('react-icons/fi', () => ({
  FiMapPin: 'FiMapPin'
}));

import { MapArea } from '../../src/gui/js/map-area'; // eslint-disable-line import/first

describe('MapArea', () => {
  test('renders a map centered at (0,0) if no photo prop is set', () => {
    const mapArea = renderer.create(<MapArea />).toJSON();
    expect(mapArea).toMatchSnapshot();
  });

  test('renders a map centered at (0,0) if the photo has no location', () => {
    const photo: Photo = {
      path: ''
    };
    const mapArea = renderer.create(<MapArea photo={photo} />).toJSON();
    expect(mapArea).toMatchSnapshot();
  });

  test("renders a map centered and with a marker at the photo's location if it has one", () => {
    const photo = {
      location: {
        Existing: {
          latitude: 52.0,
          longitude: 36.2
        }
      },
      path: ''
    };
    const mapArea = renderer.create(<MapArea photo={photo} />).toJSON();
    expect(mapArea).toMatchSnapshot();
  });
});
