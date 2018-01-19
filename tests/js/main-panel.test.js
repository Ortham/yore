import React from 'react';
import renderer from 'react-test-renderer';

jest.mock('../../src/gui/js/map-area', () => 'MapArea');

import MainPanel from '../../src/gui/js/main-panel';

function apply() {
    return 'apply';
}

function discard() {
    return 'discard';
}

describe('MainPanel', () => {
    test('renders an image, map, location description and disabled buttons if photo location is not suggested', () => {
        let photo = {
            src: 'path',
            location: {
                Existing: {
                    latitude: 52.0,
                    longitude: 36.2
                }
            }
        };
        const mainPanel = renderer.create(
            <MainPanel photo={photo}
                handleSuggestionApply={apply}
                handleSuggestionDiscard={discard}
            />
        ).toJSON();
        expect(mainPanel).toMatchSnapshot();
    });

    test('renders an image, map, text and disabled buttons if photo has no location', () => {
        let photo = {
            src: 'path'
        };
        const mainPanel = renderer.create(
            <MainPanel photo={photo}
                handleSuggestionApply={apply}
                handleSuggestionDiscard={discard}
            />
        ).toJSON();
        expect(mainPanel).toMatchSnapshot();
    });

    test('renders an image, map, location description and enabled buttons if photo location is suggested', () => {
        let photo = {
            src: 'path',
            location: {
                Suggested: [{
                    latitude: 52.0,
                    longitude: 36.2
                },{
                    meters: 5,
                    seconds: 10
                }]
            }
        };
        const mainPanel = renderer.create(
            <MainPanel photo={photo}
                handleSuggestionApply={apply}
                handleSuggestionDiscard={discard}
            />
        ).toJSON();
        expect(mainPanel).toMatchSnapshot();
    });

    test('renders text and disabled buttons if photo is undefined', () => {
        const mainPanel = renderer.create(<MainPanel />).toJSON();
        expect(mainPanel).toMatchSnapshot();
    });
});
