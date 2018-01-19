import React from 'react';
import renderer from 'react-test-renderer';

jest.mock('google-map-react', () => 'GoogleMapReact');
jest.mock('react-icons/lib/fa/map-marker', () => 'FaMapMarker');

import MapArea from '../../src/gui/js/map-area';

describe('MapArea', () => {
    test('renders a map centered at (0,0) if no photo prop is set', () => {
        const mapArea = renderer.create(<MapArea />).toJSON();
        expect(mapArea).toMatchSnapshot();
    });

    test('renders a map centered at (0,0) if the photo has no location', () => {
        let photo = {};
        const mapArea = renderer.create(<MapArea photo={photo} />).toJSON();
        expect(mapArea).toMatchSnapshot();
    });

    test('renders a map centered and with a marker at the photo\'s location if it has one', () => {
        let photo = {
            location: {
                Existing: {
                    latitude: 52.0,
                    longitude: 36.2
                }
            }
        };
        const mapArea = renderer.create(<MapArea photo={photo} />).toJSON();
        expect(mapArea).toMatchSnapshot();
    });
});
